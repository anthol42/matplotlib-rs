use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyAxhlineBuilder {
    y: f64,
    xmin: Option<f64>,
    xmax: Option<f64>,
    color: Option<String>,
    linewidth: Option<f64>,
    linestyle: Option<String>,
    alpha: Option<f64>,
    label: Option<String>,
    zorder: Option<i32>,
}

impl PyAxhlineBuilder {
    pub fn new(y: f64) -> Self {
        Self {
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

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            ax.call_method("axhline", (self.y,), Some(&kwargs))?;
            Ok(())
        })
    }
}

/// Add a horizontal line across the axes.
///
/// # Parameters
/// - `y`: f64
/// - `xmin`: Option<f64>
/// - `xmax`: Option<f64>
/// - `color`: Option<String>
/// - `linewidth`: Option<f64>
/// - `linestyle`: Option<String>
/// - `alpha`: Option<f64>
/// - `label`: Option<String>
/// - `zorder`: Option<i32>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.axhline.html>
pub fn axhline(y: f64) -> PyAxhlineBuilder {
    PyAxhlineBuilder::new(y)
}
