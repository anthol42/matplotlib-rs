use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Type};

#[proc_macro_derive(KwargsBuilder)]
pub fn kwargs_builder_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("KwargsBuilder only works with named fields"),
        },
        _ => panic!("KwargsBuilder only works with structs"),
    };

    // Generate builder methods for Option<T> fields
    let builder_methods: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref()?;
            let field_ty = &f.ty;  // Get the actual type

            // Check if the field is Option<T>
            if is_option_type(&f.ty) {
                let method_name = field_name;
                Some(quote! {
                    pub fn #method_name(mut self, value: #field_ty) -> Self {
                        self.#field_name = value;
                        self
                    }
                })
            } else {
                None
            }
        })
        .collect();

    // Generate get_kwargs method
    let kwargs_insertions: Vec<_> = fields
        .iter()
        .filter_map(|f| {
            let field_name = f.ident.as_ref()?;
            let field_name_str = field_name.to_string();

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
        impl #name {
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