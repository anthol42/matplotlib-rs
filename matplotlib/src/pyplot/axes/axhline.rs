use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct AxhlineBuilder<'a> {
    axes: &'a Axes,
    y: f64,
    // Range in axes coordinates [0, 1]
    xmin: Option<f64>,
    xmax: Option<f64>,
    // Line styling
    color: Option<String>,
    linewidth: Option<f64>,
    linestyle: Option<String>,
    alpha: Option<f64>,
    label: Option<String>,
    zorder: Option<i32>,
}

impl<'a> AxhlineBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            axes_obj.call_method("axhline", (self.y,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    pub fn axhline(&self, y: f64) -> AxhlineBuilder<'_> {
        AxhlineBuilder {
            axes: self,
            y,
            xmin: None,
            xmax: None,
            color: None,
            linewidth: None,
            linestyle: None,
            alpha: None,
            label: None,
            zorder: None,
        }
    }
}
