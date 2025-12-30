//! # KwargsBuilder Derive Macro
//!
//! A procedural macro that generates builder patterns for Python keyword arguments (kwargs)
//! when using PyO3. This macro simplifies the creation of Rust APIs that mirror Python's
//! optional parameter syntax.
//!
//! ## Overview
//!
//! When wrapping Python libraries with PyO3, Python functions often have many optional parameters.
//! This macro automates the builder pattern for such parameters, making your Rust API clean and ergonomic.
//!
//! ## Features
//!
//! - **Automatic builder methods**: Generates setters for all `Option<T>` fields
//! - **Clean API**: Setters take `T` instead of `Option<T>`, wrapping values automatically
//! - **PyO3 integration**: Generates `get_kwargs()` that creates a PyDict with only set values
//! - **Selective skipping**: Use `#[kwargs_builder(skip)]` for custom handling of complex types
//!
//! ## Quick Start
//!
//! ```ignore
//! use kwargs_builder_derive::KwargsBuilder;
//!
//! #[derive(KwargsBuilder)]
//! struct PlotBuilder {
//!     x: Vec<f64>,              // Required
//!     y: Vec<f64>,              // Required
//!     color: Option<String>,    // Optional - generates .color() method
//!     linewidth: Option<f64>,   // Optional - generates .linewidth() method
//! }
//!
//! // Usage:
//! let builder = PlotBuilder { x, y, color: None, linewidth: None };
//! builder.color("red".to_string()).linewidth(2.0);  // Clean API!
//! ```
//!
//! See the [`KwargsBuilder`](derive.KwargsBuilder.html) derive macro documentation for detailed examples.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type, Attribute};

/// Derives a builder pattern for PyO3 keyword arguments (kwargs).
///
/// This macro automatically generates:
/// 1. **Builder methods** for all `Option<T>` fields that take unwrapped `T` values
/// 2. **`get_kwargs()` method** that creates a PyO3 `PyDict` with only the set values
///
/// # What it does
///
/// For each field of type `Option<T>`:
/// - Generates a method `pub fn field_name(mut self, value: T) -> Self`
/// - The method wraps the value in `Some(value)` and stores it
/// - Adds the field to kwargs dict in `get_kwargs()` only if it's `Some`
///
/// # Requirements
///
/// - Only works with structs that have named fields
/// - Only generates methods for fields of type `Option<T>`
/// - Requires PyO3 to be in scope for the `get_kwargs()` method
///
/// # Attribute: `#[kwargs_builder(skip)]`
///
/// Use this attribute on fields you want to handle manually:
/// - Skips builder method generation for that field
/// - Excludes field from `get_kwargs()` dict insertion
/// - Useful for fields with complex types that need custom conversion logic
///
/// # Basic Example
///
/// ```ignore
/// use kwargs_builder_derive::KwargsBuilder;
///
/// #[derive(KwargsBuilder)]
/// struct PlotBuilder {
///     x: Vec<f64>,              // Required field (not Option)
///     y: Vec<f64>,              // Required field (not Option)
///     color: Option<String>,    // Optional - builder method generated
///     linewidth: Option<f64>,   // Optional - builder method generated
///     alpha: Option<f64>,       // Optional - builder method generated
/// }
///
/// // Generated code (simplified):
/// impl PlotBuilder {
///     // Builder methods for Option fields
///     pub fn color(mut self, value: String) -> Self {
///         self.color = Some(value);
///         self
///     }
///
///     pub fn linewidth(mut self, value: f64) -> Self {
///         self.linewidth = Some(value);
///         self
///     }
///
///     pub fn alpha(mut self, value: f64) -> Self {
///         self.alpha = Some(value);
///         self
///     }
///
///     // Generated get_kwargs method
///     pub fn get_kwargs<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
///         let kwargs = PyDict::new(py);
///         if let Some(ref value) = self.color {
///             kwargs.set_item("color", value)?;
///         }
///         if let Some(ref value) = self.linewidth {
///             kwargs.set_item("linewidth", value)?;
///         }
///         if let Some(ref value) = self.alpha {
///             kwargs.set_item("alpha", value)?;
///         }
///         Ok(kwargs)
///     }
/// }
///
/// // Usage:
/// let builder = PlotBuilder {
///     x: vec![1.0, 2.0, 3.0],
///     y: vec![1.0, 4.0, 9.0],
///     color: None,
///     linewidth: None,
///     alpha: None,
/// };
///
/// builder
///     .color("blue".to_string())  // No need for Some(...)!
///     .linewidth(2.0)
///     .alpha(0.7);
/// ```
///
/// # Advanced Example with `#[kwargs_builder(skip)]`
///
/// Use the `skip` attribute when you need custom type conversion logic:
///
/// ```ignore
/// use kwargs_builder_derive::KwargsBuilder;
///
/// // Custom enum for multi-type parameter
/// pub enum ScatterSize {
///     Scalar(f64),
///     Array(Array1<f64>),
/// }
///
/// #[derive(KwargsBuilder)]
/// struct ScatterBuilder {
///     x: Array1<f64>,           // Required
///     y: Array1<f64>,           // Required
///
///     // Skip auto-generation for custom handling
///     #[kwargs_builder(skip)]
///     s: Option<ScatterSize>,
///
///     // Regular optional parameters
///     marker: Option<String>,
///     alpha: Option<f64>,
/// }
///
/// impl ScatterBuilder {
///     // Manually implement generic setter for `s`
///     pub fn s<T: Into<ScatterSize>>(mut self, value: T) -> Self {
///         self.s = Some(value.into());
///         self
///     }
///
///     // In your set() method, manually handle the skipped field
///     pub fn set(self) -> PyResult<()> {
///         Python::attach(|py| {
///             let kwargs = self.get_kwargs(py)?;  // Only includes marker, alpha
///
///             // Manually add `s` with custom conversion
///             if let Some(size) = self.s {
///                 match size {
///                     ScatterSize::Scalar(val) => kwargs.set_item("s", val)?,
///                     ScatterSize::Array(arr) => kwargs.set_item("s", arr.into_pyarray(py))?,
///                 }
///             }
///
///             // ... call Python method with kwargs
///             Ok(())
///         })
///     }
/// }
///
/// // Usage with automatic type conversion:
/// builder
///     .s(100.0)                    // f64 -> ScatterSize::Scalar
///     .s(arr1(&[50.0, 100.0]))     // Array1 -> ScatterSize::Array
///     .marker("o".to_string())
///     .alpha(0.7);
/// ```
///
/// # Notes
///
/// - Non-`Option` fields are ignored by the macro
/// - The macro doesn't generate constructors - you define those yourself
/// - All generated methods follow the builder pattern (return `Self`)
/// - The `get_kwargs()` method requires `pyo3::Python`, `pyo3::types::PyDict`, and `pyo3::PyResult`
#[proc_macro_derive(KwargsBuilder, attributes(kwargs_builder))]
pub fn kwargs_builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("KwargsBuilder only works with named fields"),
        },
        _ => panic!("KwargsBuilder only works with structs"),
    };

    // Generate builder methods for Option<T> fields (unless marked with #[kwargs_builder(skip)])
    let builder_methods: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref()?;

            // Check if field has #[kwargs_builder(skip)] attribute
            if has_skip_attribute(&f.attrs) {
                return None;
            }

            // Check if the field is Option<T> and extract inner type T
            if let Some(inner_ty) = extract_option_inner_type(&f.ty) {
                let method_name = field_name;
                Some(quote! {
                    pub fn #method_name(mut self, value: #inner_ty) -> Self {
                        self.#field_name = Some(value);
                        self
                    }
                })
            } else {
                None
            }
        })
        .collect();

    // Generate get_kwargs method (skip fields marked with #[kwargs_builder(skip)])
    let kwargs_insertions: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref()?;
            let field_name_str = field_name.to_string();

            // Check if field has #[kwargs_builder(skip)] attribute
            if has_skip_attribute(&f.attrs) {
                return None;
            }

            if is_option_type(&f.ty) {
                Some(quote! {
                    if let Some(ref value) = self.#field_name {
                        kwargs.set_item(#field_name_str, value)?;
                    }
                })
            } else {
                None
            }
        })
        .collect();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#builder_methods)*

        pub fn get_kwargs<'py>(&self, py: pyo3::Python<'py>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyDict>> {
            let kwargs = pyo3::types::PyDict::new(py);
            #(#kwargs_insertions)*
            Ok(kwargs)
        }
        }
    };

    TokenStream::from(expanded)
}

fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}

fn extract_option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Option" {
                // Extract the inner type from Option<T>
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
}

fn has_skip_attribute(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("kwargs_builder") {
            // Parse the attribute content
            if let Ok(meta_list) = attr.meta.require_list() {
                // Check if it contains "skip"
                return meta_list.tokens.to_string().contains("skip");
            }
        }
        false
    })
}