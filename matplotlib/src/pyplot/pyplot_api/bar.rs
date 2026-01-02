use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;
use std::collections::HashMap;

use super::shared::get_plt;

pub enum BarWidth {
    Scalar(f64),
    Array(Array1<f64>),
}

pub enum BarBottom {
    Scalar(f64),
    Array(Array1<f64>),
}

pub enum ErrorKwValue {
    Float(f64),
    String(String),
    Bool(bool),
}

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
pub struct PyBarBuilder {
    x: Array1<f64>,
    height: Array1<f64>,
    #[kwargs_builder(skip)]
    width: Option<BarWidth>,
    #[kwargs_builder(skip)]
    bottom: Option<BarBottom>,
    align: Option<String>,
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    label: Option<String>,
    #[kwargs_builder(skip)]
    tick_label: Option<Vec<String>>,
    #[kwargs_builder(skip)]
    yerr: Option<Array1<f64>>,
    #[kwargs_builder(skip)]
    xerr: Option<Array1<f64>>,
    ecolor: Option<String>,
    capsize: Option<f64>,
    #[kwargs_builder(skip)]
    error_kw: Option<HashMap<String, ErrorKwValue>>,
    hatch: Option<String>,
    zorder: Option<i32>,
}

impl PyBarBuilder {
    pub fn new(x: Array1<f64>, height: Array1<f64>) -> Self {
        Self {
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

    pub fn width<T: Into<BarWidth>>(mut self, value: T) -> Self {
        self.width = Some(value.into());
        self
    }

    pub fn bottom<T: Into<BarBottom>>(mut self, value: T) -> Self {
        self.bottom = Some(value.into());
        self
    }

    pub fn tick_label(mut self, value: Vec<String>) -> Self {
        self.tick_label = Some(value);
        self
    }

    pub fn yerr(mut self, value: Array1<f64>) -> Self {
        self.yerr = Some(value);
        self
    }

    pub fn xerr(mut self, value: Array1<f64>) -> Self {
        self.xerr = Some(value);
        self
    }

    pub fn error_kw(mut self, value: HashMap<String, ErrorKwValue>) -> Self {
        self.error_kw = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            let x_numpy = self.x.into_pyarray(py);
            let height_numpy = self.height.into_pyarray(py);

            if let Some(width_value) = self.width {
                match width_value {
                    BarWidth::Scalar(val) => kwargs.set_item("width", val)?,
                    BarWidth::Array(arr) => kwargs.set_item("width", arr.into_pyarray(py))?,
                }
            }

            if let Some(bottom_value) = self.bottom {
                match bottom_value {
                    BarBottom::Scalar(val) => kwargs.set_item("bottom", val)?,
                    BarBottom::Array(arr) => kwargs.set_item("bottom", arr.into_pyarray(py))?,
                }
            }

            if let Some(labels) = self.tick_label {
                kwargs.set_item("tick_label", labels)?;
            }

            if let Some(yerr_arr) = self.yerr {
                kwargs.set_item("yerr", yerr_arr.into_pyarray(py))?;
            }

            if let Some(xerr_arr) = self.xerr {
                kwargs.set_item("xerr", xerr_arr.into_pyarray(py))?;
            }

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

            ax.call_method("bar", (x_numpy, height_numpy), Some(&kwargs))?;
            Ok(())
        })
    }
}

pub fn bar(x: Array1<f64>, height: Array1<f64>) -> PyBarBuilder {
    PyBarBuilder::new(x, height)
}
