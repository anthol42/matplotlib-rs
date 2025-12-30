use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

// Enum for y2 parameter (can be scalar or array)
pub enum FillBetweenY2 {
    Scalar(f64),
    Array(Array1<f64>),
}

// Implement From traits for automatic type conversion - FillBetweenY2
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
    // Multi-type parameter (handled manually with custom setter)
    #[kwargs_builder(skip)]
    y2: Option<FillBetweenY2>,
    // Conditional filling (where_mask handled manually since it needs conversion to numpy array)
    #[kwargs_builder(skip)]
    where_mask: Option<Array1<bool>>,
    interpolate: Option<bool>,
    // Visual styling
    alpha: Option<f64>,
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    // Step functions
    step: Option<String>,
    // General
    label: Option<String>,
    zorder: Option<i32>,
    hatch: Option<String>,
}

impl<'a> FillBetweenBuilder<'a> {
    // Manual method for y2 with generic type conversion
    pub fn y2<T: Into<FillBetweenY2>>(mut self, value: T) -> Self {
        self.y2 = Some(value.into());
        self
    }

    // Manual method for where_mask (named 'where_mask' in Rust, 'where' in Python)
    pub fn where_mask(mut self, value: Array1<bool>) -> Self {
        self.where_mask = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert Array1 to NumPy arrays (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);
            let y1_numpy = self.y1.into_pyarray(py);

            // Handle y2 parameter manually (skipped by KwargsBuilder)
            if let Some(y2_value) = self.y2 {
                match y2_value {
                    FillBetweenY2::Scalar(val) => kwargs.set_item("y2", val)?,
                    FillBetweenY2::Array(arr) => kwargs.set_item("y2", arr.into_pyarray(py))?,
                }
            }

            // Handle where_mask - rename to "where" for Python
            // Note: We use "where_mask" in Rust because "where" is a keyword
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
