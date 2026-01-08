use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct AxvlineBuilder<'a> {
    axes: &'a Axes,
    x: f64,
    // Range in axes coordinates [0, 1]
    ymin: Option<f64>,
    ymax: Option<f64>,
    // Line styling
    color: Option<String>,
    linewidth: Option<f64>,
    linestyle: Option<String>,
    alpha: Option<f64>,
    label: Option<String>,
    zorder: Option<i32>,
}

impl<'a> AxvlineBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            axes_obj.call_method("axvline", (self.x,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Add a vertical line across the axes.
    ///
    /// # Parameters
    /// - `x`: f64
    /// - `ymin`: Option<f64>
    /// - `ymax`: Option<f64>
    /// - `color`: Option<String>
    /// - `linewidth`: Option<f64>
    /// - `linestyle`: Option<String>
    /// - `alpha`: Option<f64>
    /// - `label`: Option<String>
    /// - `zorder`: Option<i32>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.axvline.html>
    pub fn axvline(&self, x: f64) -> AxvlineBuilder<'_> {
        AxvlineBuilder {
            axes: self,
            x,
            ymin: None,
            ymax: None,
            color: None,
            linewidth: None,
            linestyle: None,
            alpha: None,
            label: None,
            zorder: None,
        }
    }
}
