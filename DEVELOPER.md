# Developer Guide for matplotlib-rs

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture](#architecture)
3. [Core Concepts](#core-concepts)
4. [Directory Structure](#Organization-Principles)
5. [How to Add a New Function](#how-to-add-a-new-function)
6. [Code Patterns and Examples](#code-patterns-and-examples)
7. [Best Practices](#best-practices)
8. [Building and Testing](#building-and-testing)

---

## Project Overview

**matplotlib-rs** is a Rust wrapper for Python's matplotlib library using PyO3. The primary goal is to **mirror the true matplotlib Python API** as closely as possible, providing Rust developers with a familiar, idiomatic interface to matplotlib's plotting capabilities.

### Design Philosophy

- **API Parity**: Every function and parameter should match matplotlib's Python API
- **Type Safety**: Use Rust's type system to catch errors at compile time
- **Ergonomics**: Leverage Rust patterns (builder pattern, `.into()` conversions) for clean code
- **Zero Copies**: Use move semantics where possible to avoid unnecessary data copying

---

## Architecture

The library uses PyO3 to bridge Rust and Python, with three main API surfaces:

1. **`pyplot` API** (`plt.plot()`, `plt.scatter()`, etc.) - Stateful API using current axes/figure
2. **`Axes` methods** (`axes.plot()`, `axes.scatter()`, etc.) - Instance methods on specific axes
3. **`Figure` methods** (`figure.suptitle()`, `figure.savefig()`, etc.) - Instance methods on figures

### Key Architectural Patterns

#### 1. Builder Pattern with Deferred Execution

All functions with optional parameters use the **builder pattern**. Python code execution is **deferred** until `.set()` is called.

```rust
// Python equivalent: plt.plot(x, y, color='red', linewidth=2.0)

// Rust: Function returns a builder
let builder = plt::plot(x, y)       // Returns PyPlotBuilder
    .color("red".to_string())        // Optional parameter
    .linewidth(2.0);                 // Optional parameter

// Python code executes ONLY when .set() is called
builder.set()?;
```

**Why deferred execution?**
- Allows collecting all parameters before making expensive Python FFI call
- Enables clean builder API without nested function calls
- Matches matplotlib's flexibility with optional parameters

#### 2. KwargsBuilder Derive Macro

The `#[derive(KwargsBuilder)]` procedural macro automates the builder pattern:

```rust
#[derive(KwargsBuilder)]
pub struct PyPlotBuilder {
    x: Array1<f64>,              // Required field (not Option)
    y: Array1<f64>,              // Required field (not Option)
    color: Option<String>,       // Optional - generates .color() method
    linewidth: Option<f64>,      // Optional - generates .linewidth() method
    alpha: Option<f64>,          // Optional - generates .alpha() method
}
```

The macro **automatically generates**:
- **Setter methods** for each `Option<T>` field (e.g., `.color(value: String)`)
- **`get_kwargs(py)` method** that creates a PyDict with only the `Some` values

See [`kwargs_builder_derive/src/lib.rs`](kwargs_builder_derive/src/lib.rs) for full documentation.

#### 3. Shared PLT Instance

All pyplot functions (`plt.plot()`, `plt.scatter()`, etc.) share a **single static pyplot module instance** to avoid redundant Python imports:

```rust
// src/pyplot/pyplot_api/shared.rs
static PLT: OnceLock<Py<PyAny>> = OnceLock::new();

pub fn get_plt() -> PyResult<Py<PyAny>> {
    Python::attach(|py| {
        if let Some(plt) = PLT.get() {
            Ok(plt.clone_ref(py))
        } else {
            let plt_module = py.import("matplotlib.pyplot")?;
            let plt: Py<PyAny> = plt_module.unbind().into();
            PLT.set(plt.clone_ref(py)).ok();
            Ok(plt)
        }
    })
}
```

**Thread-safe singleton pattern** using `OnceLock` ensures matplotlib.pyplot is imported only once.

#### 4. Supporting Multiple Types with Enums and `.into()`

Python functions often accept multiple types for a parameter (e.g., `y2` can be a scalar or array). We use **enums** with **From trait implementations** for ergonomic APIs:

```rust
// Example from fill_between.rs
pub enum FillBetweenY2 {
    Scalar(f64),
    Array(Array1<f64>),
}

// Implement From traits for automatic conversion
impl From<f64> for FillBetweenY2 {
    fn from(value: f64) -> Self {
        FillBetweenY2::Scalar(value)
    }
}

impl From<Array1<f64>> for FillBetweenY2 {
    fn from(value: Array1<f64>) -> Self {
        FillBetweenY2::Array(value)
    }
}
```

**Generic setter method** accepts any type that implements `Into<FillBetweenY2>`:

```rust
#[derive(KwargsBuilder)]
pub struct FillBetweenBuilder {
    #[kwargs_builder(skip)]  // Skip auto-generation for custom handling
    y2: Option<FillBetweenY2>,
}

impl FillBetweenBuilder {
    // Generic method - accepts both f64 and Array1<f64>!
    pub fn y2<T: Into<FillBetweenY2>>(mut self, value: T) -> Self {
        self.y2 = Some(value.into());
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let kwargs = self.get_kwargs(py)?;

            // Manually convert enum to Python type
            if let Some(y2_value) = self.y2 {
                match y2_value {
                    FillBetweenY2::Scalar(val) => kwargs.set_item("y2", val)?,
                    FillBetweenY2::Array(arr) => kwargs.set_item("y2", arr.into_pyarray(py))?,
                }
            }

            // ... call Python method
            Ok(())
        })
    }
}
```

**Usage** - both work seamlessly:
```rust
builder.y2(0.0)           // f64 -> FillBetweenY2::Scalar
builder.y2(arr1(&[...]))  // Array1<f64> -> FillBetweenY2::Array
```

---


## Organization Principles

1. **One file per function** - Each matplotlib function gets its own file (e.g., `plot.rs`, `scatter.rs`)
2. **Axes vs pyplot separation** - `axes/plot.rs` is the instance method, `pyplot_api/plot.rs` is the stateful function
3. **Shared utilities** - `pyplot_api/shared.rs` contains the singleton PLT instance

---

## How to Add a New Function

Follow these steps to add a new matplotlib function (example: `plt.errorbar()`):

### Step 1: Check matplotlib documentation

Visit [matplotlib.org/stable/api](https://matplotlib.org/stable/api/) and find:
- Required parameters
- Optional parameters (these become `Option<T>` fields)
- Parameter types and valid values

### Step 2: Decide if it's an Axes, Figure or PyPlot method

- **Axes methods**: Plotting functions (plot, scatter, bar, etc.)
- **Figure methods**: Figure-level operations (suptitle, savefig, tight_layout, etc.)

### Step 3A: Add Axes Method

Create `src/pyplot/axes/errorbar.rs`:

```rust
use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct ErrorbarBuilder<'a> {
    axes: &'a Axes,
    // Required parameters (NOT Option<T>)
    x: Array1<f64>,
    y: Array1<f64>,
    // Optional parameters (as Option<T>)
    yerr: Option<f64>,        // Simple type - auto-generated setter
    xerr: Option<f64>,
    fmt: Option<String>,
    ecolor: Option<String>,
    elinewidth: Option<f64>,
    capsize: Option<f64>,
    // ... add all matplotlib parameters
}

impl<'a> ErrorbarBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert arrays to numpy
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            // Call Python method
            axes_obj.call_method("errorbar", (x_numpy, y_numpy), Some(&kwargs))?;
            Ok(())
        })
    }
}

impl Axes {
    pub fn errorbar(&self, x: Array1<f64>, y: Array1<f64>) -> ErrorbarBuilder<'_> {
        ErrorbarBuilder {
            axes: self,
            x,
            y,
            yerr: None,
            xerr: None,
            fmt: None,
            ecolor: None,
            elinewidth: None,
            capsize: None,
            // ... initialize all Option fields to None
        }
    }
}
```

**Register in `src/pyplot/axes.rs`**:
```rust
pub mod errorbar;

// At the bottom, no re-exports needed (they're in impl Axes)
```

### Step 3B: Add pyplot API Function

Create `src/pyplot/pyplot_api/errorbar.rs`:

```rust
use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::shared::get_plt;  // Import shared PLT instance

#[derive(KwargsBuilder)]
pub struct PyErrorbarBuilder {
    // IMPORTANT: pyplot builders OWN their data (no lifetime, no &Axes)
    x: Array1<f64>,
    y: Array1<f64>,
    yerr: Option<f64>,
    xerr: Option<f64>,
    fmt: Option<String>,
    ecolor: Option<String>,
    elinewidth: Option<f64>,
    capsize: Option<f64>,
    // ... same parameters as Axes version
}

impl PyErrorbarBuilder {
    pub fn new(x: Array1<f64>, y: Array1<f64>) -> Self {
        Self {
            x,
            y,
            yerr: None,
            xerr: None,
            fmt: None,
            ecolor: None,
            elinewidth: None,
            capsize: None,
            // ... initialize all to None
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;           // Get shared pyplot instance
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;  // Get current axes
            let kwargs = self.get_kwargs(py)?;

            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            ax.call_method("errorbar", (x_numpy, y_numpy), Some(&kwargs))?;
            Ok(())
        })
    }
}

pub fn errorbar(x: Array1<f64>, y: Array1<f64>) -> PyErrorbarBuilder {
    PyErrorbarBuilder::new(x, y)
}
```

**Register in `src/pyplot/pyplot_api/mod.rs`**:
```rust
pub mod errorbar;

// Re-exports
pub use errorbar::{PyErrorbarBuilder, errorbar};
```

**Export publicly in `src/pyplot.rs`**:
```rust
pub use pyplot_api::{
    // Functions
    plot, scatter, /* ... */, errorbar,  // Add here

    // Builders
    PyPlotBuilder, PyScatterBuilder, /* ... */, PyErrorbarBuilder,  // Add here
};
```

### Step 4: Handle Complex Multi-Type Parameters

If a parameter accepts multiple types (like `fill_between.y2` accepts float or array):

1. **Define an enum** for the parameter
2. **Implement `From` traits** for each variant
3. **Skip auto-generation** with `#[kwargs_builder(skip)]`
4. **Manually implement generic setter** using `.into()`
5. **Manually add to kwargs** in `.set()` method

See [fill_between example](#example-2-complex-function-with-multi-type-parameters-fill_between) below.

---

## Code Patterns and Examples

### Example 1: Simple Function (plot)

From `src/pyplot/axes/plot.rs`:

```rust
#[derive(KwargsBuilder)]
pub struct PlotBuilder<'a> {
    axes: &'a Axes,           // Lifetime: borrows axes
    x: Array1<f64>,           // Owned data
    y: Array1<f64>,           // Owned data
    // All optional parameters as Option<T>
    fmt: Option<String>,
    color: Option<String>,
    linestyle: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    label: Option<String>,
    // ... etc
}

impl<'a> PlotBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;  // Auto-generated by macro

            // Remove fmt from kwargs (it's a positional arg in matplotlib)
            let _ = kwargs.del_item("fmt");

            // Move arrays to Python
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            // Call with or without fmt
            if let Some(ref fmt_str) = self.fmt {
                axes_obj.call_method("plot", (x_numpy, y_numpy, fmt_str), Some(&kwargs))?;
            } else {
                axes_obj.call_method("plot", (x_numpy, y_numpy), Some(&kwargs))?;
            }

            Ok(())
        })
    }
}

impl Axes {
    pub fn plot(&self, x: Array1<f64>, y: Array1<f64>) -> PlotBuilder<'_> {
        PlotBuilder {
            axes: self,
            x,
            y,
            fmt: None,
            color: None,
            linestyle: None,
            // ... all Options to None
        }
    }
}
```

### Example 2: Complex Function with Multi-Type Parameters (fill_between)

From `src/pyplot/axes/fill_between.rs`:

```rust
// Enum for parameter that accepts multiple types
pub enum FillBetweenY2 {
    Scalar(f64),
    Array(Array1<f64>),
}

// Implement From traits for ergonomic .into() conversion
impl From<f64> for FillBetweenY2 {
    fn from(value: f64) -> Self {
        FillBetweenY2::Scalar(value)
    }
}

impl From<Array1<f64>> for FillBetweenY2 {
    fn from(value: Array1<f64>) -> Self {
        FillBetweenY2::Array(value)
    }
}

#[derive(KwargsBuilder)]
pub struct FillBetweenBuilder<'a> {
    axes: &'a Axes,
    x: Array1<f64>,
    y1: Array1<f64>,

    // Skip auto-generation for multi-type parameter
    #[kwargs_builder(skip)]
    y2: Option<FillBetweenY2>,

    // Skip auto-generation for parameters that need renaming
    #[kwargs_builder(skip)]
    where_mask: Option<Array1<bool>>,  // 'where' in Python, keyword in Rust

    // Regular optional parameters (auto-generated setters)
    interpolate: Option<bool>,
    alpha: Option<f64>,
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    step: Option<String>,
    label: Option<String>,
    zorder: Option<i32>,
    hatch: Option<String>,
}

impl<'a> FillBetweenBuilder<'a> {
    // Manual generic setter for y2
    pub fn y2<T: Into<FillBetweenY2>>(mut self, value: T) -> Self {
        self.y2 = Some(value.into());
        self
    }

    // Manual setter for where_mask (Rust keyword workaround)
    pub fn where_mask(mut self, value: Array1<bool>) -> Self {
        self.where_mask = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;  // Only includes non-skipped fields

            let x_numpy = self.x.into_pyarray(py);
            let y1_numpy = self.y1.into_pyarray(py);

            // Manually handle y2 (skipped by KwargsBuilder)
            if let Some(y2_value) = self.y2 {
                match y2_value {
                    FillBetweenY2::Scalar(val) => kwargs.set_item("y2", val)?,
                    FillBetweenY2::Array(arr) => kwargs.set_item("y2", arr.into_pyarray(py))?,
                }
            }

            // Manually handle where_mask - rename to "where" for Python
            if let Some(mask) = self.where_mask {
                kwargs.set_item("where", mask.into_pyarray(py))?;
            }

            axes_obj.call_method("fill_between", (x_numpy, y1_numpy), Some(&kwargs))?;
            Ok(())
        })
    }
}

impl Axes {
    pub fn fill_between(&self, x: Array1<f64>, y1: Array1<f64>) -> FillBetweenBuilder<'_> {
        FillBetweenBuilder {
            axes: self,
            x,
            y1,
            y2: None,
            where_mask: None,
            interpolate: None,
            alpha: None,
            color: None,
            edgecolor: None,
            linewidth: None,
            step: None,
            label: None,
            zorder: None,
            hatch: None,
        }
    }
}
```

**Usage**:
```rust
// Both work thanks to .into() conversion
axes.fill_between(x, y1)
    .y2(0.0)                      // f64 -> FillBetweenY2::Scalar
    .color("blue".to_string())
    .set()?;

axes.fill_between(x, y1)
    .y2(y2_array)                 // Array1<f64> -> FillBetweenY2::Array
    .alpha(0.3)
    .set()?;
```

### Example 3: pyplot API (Stateful)

From `src/pyplot/pyplot_api/plot.rs`:

```rust
use super::shared::get_plt;  // Shared PLT instance

#[derive(KwargsBuilder)]
pub struct PyPlotBuilder {
    // OWNS data (no lifetime, no &Axes reference)
    x: Array1<f64>,
    y: Array1<f64>,
    // Same optional parameters as Axes::plot
    fmt: Option<String>,
    color: Option<String>,
    linestyle: Option<String>,
    // ... etc (MUST match Axes version exactly)
}

impl PyPlotBuilder {
    pub fn new(x: Array1<f64>, y: Array1<f64>) -> Self {
        Self {
            x,
            y,
            fmt: None,
            color: None,
            linestyle: None,
            // ... all to None
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;           // Get shared singleton
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;  // Get current axes
            let kwargs = self.get_kwargs(py)?;

            let _ = kwargs.del_item("fmt");
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            if let Some(ref fmt_str) = self.fmt {
                ax.call_method("plot", (x_numpy, y_numpy, fmt_str), Some(&kwargs))?;
            } else {
                ax.call_method("plot", (x_numpy, y_numpy), Some(&kwargs))?;
            }

            Ok(())
        })
    }
}

pub fn plot(x: Array1<f64>, y: Array1<f64>) -> PyPlotBuilder {
    PyPlotBuilder::new(x, y)
}
```

**Key Differences**:
- **Owned data**: `x: Array1<f64>` not `x: &Array1<f64>`
- **No axes reference**: Builder doesn't hold `&Axes`
- **Uses `get_plt()`**: Shared pyplot singleton
- **Calls `gca()`**: Gets current axes at `.set()` time

### Example 4: Figure Methods

From `src/pyplot/figure/suptitle.rs`:

```rust
#[derive(KwargsBuilder)]
pub struct SuptitleBuilder<'a> {
    figure: &'a Figure,      // References Figure, not Axes
    t: String,               // Required text parameter
    x: Option<f64>,
    y: Option<f64>,
    // ... optional parameters
}

impl<'a> SuptitleBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let fig_obj = self.figure.inner.bind(py);  // Use figure, not axes
            let kwargs = self.get_kwargs(py)?;

            fig_obj.call_method("suptitle", (self.t,), Some(&kwargs))?;
            Ok(())
        })
    }
}

impl Figure {
    pub fn suptitle(&self, t: String) -> SuptitleBuilder<'_> {
        SuptitleBuilder {
            figure: self,
            t,
            x: None,
            y: None,
            // ...
        }
    }
}
```

---

## Best Practices

### 1. Always Match matplotlib's API Exactly

**Goal**: A Python developer should feel at home with the Rust API.

```rust
// Python
plt.plot(x, y, color='red', linewidth=2.0, alpha=0.5)

// Rust (should feel similar)
plt::plot(x, y)
    .color("red".to_string())
    .linewidth(2.0)
    .alpha(0.5)
    .set()?;
```

### 2. Use Enums for Type-Safe Options

Instead of accepting arbitrary strings, use enums:

```rust
// BAD
builder.legend_loc("upper right".to_string())

// GOOD
pub enum LegendLoc {
    Best, UpperRight, UpperLeft, /* ... */
}

builder.legend_loc(LegendLoc::UpperRight)
```

### 3. Shared PLT Instance for pyplot Functions

**Always** use `use super::shared::get_plt;` in pyplot_api files:

```rust
// CORRECT
use super::shared::get_plt;

pub fn set(self) -> PyResult<()> {
    Python::attach(|py| {
        let plt_obj = get_plt()?;  // Shared singleton
        // ...
    })
}
```

**Never** create a new static PLT in individual files:

```rust
// WRONG - Don't do this!
static PLT: OnceLock<Py<PyAny>> = OnceLock::new();
```

### 4. Parameter Parity Between Axes and pyplot

pyplot builders **MUST** have identical parameters to their Axes counterparts:

```rust
// axes/plot.rs
pub struct PlotBuilder<'a> {
    axes: &'a Axes,
    x: Array1<f64>,
    y: Array1<f64>,
    color: Option<String>,
    linewidth: Option<f64>,
    // ... 15 more parameters
}

// pyplot_api/plot.rs
pub struct PyPlotBuilder {
    // NO axes field, but ALL the same optional parameters
    x: Array1<f64>,
    y: Array1<f64>,
    color: Option<String>,
    linewidth: Option<f64>,
    // ... SAME 15 parameters
}
```

### 5. Use `#[kwargs_builder(skip)]` Sparingly

Only skip fields when:
- The parameter accepts multiple types (use enum + `.into()`)
- The parameter name is a Rust keyword (e.g., `where` → `where_mask`)
- Custom conversion logic is required

Otherwise, let the macro do the work.

### 6. Ownership and Lifetimes

**Axes methods**:
```rust
pub struct PlotBuilder<'a> {
    axes: &'a Axes,  // Borrow axes with lifetime
    x: Array1<f64>,  // Own the data
}
```

**pyplot functions**:
```rust
pub struct PyPlotBuilder {
    // No axes field at all
    x: Array1<f64>,  // Own the data
}
```

### 7. Move, Don't Clone

Use `.into_pyarray(py)` which **moves** ownership:

```rust
// GOOD - moves ownership
let x_numpy = self.x.into_pyarray(py);

// AVOID - unnecessary clone
let x_numpy = self.x.clone().into_pyarray(py);
```

---

## Building and Testing

### Build Commands

```bash
# Build the entire workspace (library + proc macro)
cargo build

# Build specific crate
cargo build -p matplotlib
cargo build -p kwargs_builder_derive

# Run the example binary
cargo run --example <example_name>

# Check for errors (faster than build)
cargo check
```

### Testing Pattern

Currently, testing is done via the example binary in `src/main.rs`:

```rust
use matplotlib::pyplot as plt;
use ndarray::arr1;

fn main() -> PyResult<()> {
    let x = arr1(&[1.0, 2.0, 3.0, 4.0]);
    let y = arr1(&[1.0, 4.0, 9.0, 16.0]);

    plt::plot(x, y)
        .color("blue".to_string())
        .linewidth(2.0)
        .set()?;

    plt::show()?;
    Ok(())
}
```

Run with: `cargo run --bin matplotlib`

### Common Issues

**Issue**: "Python matplotlib not found"
- **Fix**: Install matplotlib in your Python environment: `pip install matplotlib`

**Issue**: "Rust version too old"
- **Fix**: Update to Rust 1.85.0+ (project uses edition 2024)

**Issue**: "Lifetime errors with builders"
- **Fix**: Ensure Axes methods use `'a` lifetime, pyplot methods don't

---

## Quick Reference Checklist

When adding a new function, ask yourself:

- [ ] Is this an Axes method, Figure method, or both?
- [ ] Did I check matplotlib docs for ALL parameters?
- [ ] Did I use `#[derive(KwargsBuilder)]`?
- [ ] Are all optional parameters `Option<T>`?
- [ ] For multi-type parameters, did I create enums with `From` traits?
- [ ] Did I implement the `.set()` method with `Python::attach`?
- [ ] For pyplot API, did I use `get_plt()` from `super::shared`?
- [ ] For pyplot API, did I call `.gca()` or `.gcf()` to get current axes/figure?
- [ ] Did I register the module in `axes.rs` / `figure.rs` / `pyplot_api/mod.rs`?
- [ ] Did I export publicly in `pyplot.rs`?
- [ ] Does the Rust API mirror the Python API exactly?

---

## Contributing

When submitting a PR:

1. **Follow existing patterns** - consistency is key
2. **Match matplotlib's API** - check Python docs thoroughly
3. **Add examples** - create or modify `examples/*.rs` to demonstrate new functions
4. **Test your code** - run `cargo run --bin matplotlib` to verify
5. **Document everything** - Add a description for each implemented parameter, and add a link in the doc to the original matplotlib documentation

---

## Resources

- **matplotlib Python docs**: https://matplotlib.org/stable/api/
- **PyO3 documentation**: https://pyo3.rs/
- **KwargsBuilder macro**: See `kwargs_builder_derive/src/lib.rs`
- **ndarray docs**: https://docs.rs/ndarray/

---

**Happy coding! If you have questions, check existing implementations (like `fill_between.rs` for complex examples) or refer to this guide.**
