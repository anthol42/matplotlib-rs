use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::{Array1, Array2};
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

// Enum for scatter size parameter (can be scalar or array)
pub enum ScatterSize {
    Scalar(f64),
    Array(Array1<f64>),
}

// Enum for scatter color parameter (can be string, values array, or RGB/RGBA array)
pub enum ScatterColor {
    Single(String),
    Values(Array1<f64>),
    Rgb(Array2<f64>),
}

// Implement From traits for automatic type conversion - ScatterSize
impl From<f64> for ScatterSize {
    fn from(value: f64) -> Self {
        ScatterSize::Scalar(value)
    }
}

impl From<Array1<f64>> for ScatterSize {
    fn from(value: Array1<f64>) -> Self {
        ScatterSize::Array(value)
    }
}

// Implement From traits for automatic type conversion - ScatterColor
impl From<&str> for ScatterColor {
    fn from(value: &str) -> Self {
        ScatterColor::Single(value.to_string())
    }
}

impl From<String> for ScatterColor {
    fn from(value: String) -> Self {
        ScatterColor::Single(value)
    }
}

impl From<Array1<f64>> for ScatterColor {
    fn from(value: Array1<f64>) -> Self {
        ScatterColor::Values(value)
    }
}

impl From<Array2<f64>> for ScatterColor {
    fn from(value: Array2<f64>) -> Self {
        ScatterColor::Rgb(value)
    }
}

#[derive(KwargsBuilder)]
pub struct ScatterBuilder<'a> {
    axes: &'a Axes,
    x: Array1<f64>,
    y: Array1<f64>,
    // Multi-type parameters (handled manually with custom setters)
    #[kwargs_builder(skip)]
    s: Option<ScatterSize>,
    #[kwargs_builder(skip)]
    c: Option<ScatterColor>,
    // Simple optional parameters
    marker: Option<String>,
    alpha: Option<f64>,
    edgecolors: Option<String>,
    linewidths: Option<f64>,
    // Colormap parameters (for c with Values)
    cmap: Option<String>,
    vmin: Option<f64>,
    vmax: Option<f64>,
    // General
    label: Option<String>,
    zorder: Option<i32>,
    rasterized: Option<bool>,
}

impl<'a> ScatterBuilder<'a> {
    // Override the auto-generated methods for s and c to accept generic types
    pub fn s<T: Into<ScatterSize>>(mut self, value: T) -> Self {
        self.s = Some(value.into());
        self
    }

    pub fn c<T: Into<ScatterColor>>(mut self, value: T) -> Self {
        self.c = Some(value.into());
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert Array1 to NumPy arrays (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            // Handle s parameter manually (skipped by KwargsBuilder)
            if let Some(size) = self.s {
                match size {
                    ScatterSize::Scalar(val) => kwargs.set_item("s", val)?,
                    ScatterSize::Array(arr) => kwargs.set_item("s", arr.into_pyarray(py))?,
                }
            }

            // Handle c parameter manually (skipped by KwargsBuilder)
            if let Some(color) = self.c {
                match color {
                    ScatterColor::Single(val) => kwargs.set_item("c", val)?,
                    ScatterColor::Values(arr) => kwargs.set_item("c", arr.into_pyarray(py))?,
                    ScatterColor::Rgb(arr) => kwargs.set_item("c", arr.into_pyarray(py))?,
                }
            }

            axes_obj.call_method("scatter", (x_numpy, y_numpy), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    pub fn scatter(&self, x: Array1<f64>, y: Array1<f64>) -> ScatterBuilder<'_> {
        ScatterBuilder {
            axes: self,
            x,
            y,
            s: None,
            c: None,
            marker: None,
            alpha: None,
            edgecolors: None,
            linewidths: None,
            cmap: None,
            vmin: None,
            vmax: None,
            label: None,
            zorder: None,
            rasterized: None,
        }
    }
}
