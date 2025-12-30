use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;
use std::collections::HashMap;

use super::Axes;

// Enum for width parameter (can be scalar or array)
pub enum BarWidth {
    Scalar(f64),
    Array(Array1<f64>),
}

// Enum for bottom parameter (can be scalar or array)
pub enum BarBottom {
    Scalar(f64),
    Array(Array1<f64>),
}

// Enum for error_kw dictionary values (supports common types)
pub enum ErrorKwValue {
    Float(f64),
    String(String),
    Bool(bool),
}

// Implement From traits for automatic type conversion - BarWidth
impl From<f64> for BarWidth {
    fn from(value: f64) -> Self {
        BarWidth::Scalar(value)
    }
}

impl From<Array1<f64>> for BarWidth {
    fn from(value: Array1<f64>) -> Self {
        BarWidth::Array(value)
    }
}

// Implement From traits for automatic type conversion - BarBottom
impl From<f64> for BarBottom {
    fn from(value: f64) -> Self {
        BarBottom::Scalar(value)
    }
}

impl From<Array1<f64>> for BarBottom {
    fn from(value: Array1<f64>) -> Self {
        BarBottom::Array(value)
    }
}

// Implement From traits for ErrorKwValue
impl From<f64> for ErrorKwValue {
    fn from(value: f64) -> Self {
        ErrorKwValue::Float(value)
    }
}

impl From<String> for ErrorKwValue {
    fn from(value: String) -> Self {
        ErrorKwValue::String(value)
    }
}

impl From<&str> for ErrorKwValue {
    fn from(value: &str) -> Self {
        ErrorKwValue::String(value.to_string())
    }
}

impl From<bool> for ErrorKwValue {
    fn from(value: bool) -> Self {
        ErrorKwValue::Bool(value)
    }
}

#[derive(KwargsBuilder)]
pub struct BarBuilder<'a> {
    axes: &'a Axes,
    x: Array1<f64>,
    height: Array1<f64>,
    // Multi-type parameters (handled manually with custom setters)
    #[kwargs_builder(skip)]
    width: Option<BarWidth>,
    #[kwargs_builder(skip)]
    bottom: Option<BarBottom>,
    // Positioning & Alignment
    align: Option<String>,
    // Visual Styling
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    // Labeling
    label: Option<String>,
    #[kwargs_builder(skip)]
    tick_label: Option<Vec<String>>,
    // Error Bars (handled manually for array conversion)
    #[kwargs_builder(skip)]
    yerr: Option<Array1<f64>>,
    #[kwargs_builder(skip)]
    xerr: Option<Array1<f64>>,
    ecolor: Option<String>,
    capsize: Option<f64>,
    #[kwargs_builder(skip)]
    error_kw: Option<HashMap<String, ErrorKwValue>>,
    // Other Styling
    hatch: Option<String>,
    zorder: Option<i32>,
}

impl<'a> BarBuilder<'a> {
    // Manual methods for multi-type parameters with generic type conversion
    pub fn width<T: Into<BarWidth>>(mut self, value: T) -> Self {
        self.width = Some(value.into());
        self
    }

    pub fn bottom<T: Into<BarBottom>>(mut self, value: T) -> Self {
        self.bottom = Some(value.into());
        self
    }

    // Manual method for tick_label
    pub fn tick_label(mut self, value: Vec<String>) -> Self {
        self.tick_label = Some(value);
        self
    }

    // Manual methods for error bars
    pub fn yerr(mut self, value: Array1<f64>) -> Self {
        self.yerr = Some(value);
        self
    }

    pub fn xerr(mut self, value: Array1<f64>) -> Self {
        self.xerr = Some(value);
        self
    }

    // Manual method for error_kw
    pub fn error_kw(mut self, value: HashMap<String, ErrorKwValue>) -> Self {
        self.error_kw = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert Array1 to NumPy arrays (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);
            let height_numpy = self.height.into_pyarray(py);

            // Handle width parameter manually (skipped by KwargsBuilder)
            if let Some(width_value) = self.width {
                match width_value {
                    BarWidth::Scalar(val) => kwargs.set_item("width", val)?,
                    BarWidth::Array(arr) => kwargs.set_item("width", arr.into_pyarray(py))?,
                }
            }

            // Handle bottom parameter manually (skipped by KwargsBuilder)
            if let Some(bottom_value) = self.bottom {
                match bottom_value {
                    BarBottom::Scalar(val) => kwargs.set_item("bottom", val)?,
                    BarBottom::Array(arr) => kwargs.set_item("bottom", arr.into_pyarray(py))?,
                }
            }

            // Handle tick_label
            if let Some(labels) = self.tick_label {
                kwargs.set_item("tick_label", labels)?;
            }

            // Handle error bars
            if let Some(yerr_arr) = self.yerr {
                kwargs.set_item("yerr", yerr_arr.into_pyarray(py))?;
            }

            if let Some(xerr_arr) = self.xerr {
                kwargs.set_item("xerr", xerr_arr.into_pyarray(py))?;
            }

            // Handle error_kw dictionary
            if let Some(error_kw_map) = self.error_kw {
                let error_kw_dict = PyDict::new(py);
                for (key, value) in error_kw_map {
                    match value {
                        ErrorKwValue::Float(val) => error_kw_dict.set_item(key, val)?,
                        ErrorKwValue::String(val) => error_kw_dict.set_item(key, val)?,
                        ErrorKwValue::Bool(val) => error_kw_dict.set_item(key, val)?,
                    }
                }
                kwargs.set_item("error_kw", error_kw_dict)?;
            }

            axes_obj.call_method("bar", (x_numpy, height_numpy), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    pub fn bar(&self, x: Array1<f64>, height: Array1<f64>) -> BarBuilder<'_> {
        BarBuilder {
            axes: self,
            x,
            height,
            width: None,
            bottom: None,
            align: None,
            color: None,
            edgecolor: None,
            linewidth: None,
            alpha: None,
            label: None,
            tick_label: None,
            yerr: None,
            xerr: None,
            ecolor: None,
            capsize: None,
            error_kw: None,
            hatch: None,
            zorder: None,
        }
    }
}
