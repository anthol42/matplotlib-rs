use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyAxvlineBuilder {
    x: f64,
    ymin: Option<f64>,
    ymax: Option<f64>,
    color: Option<String>,
    linewidth: Option<f64>,
    linestyle: Option<String>,
    alpha: Option<f64>,
    label: Option<String>,
    zorder: Option<i32>,
}

impl PyAxvlineBuilder {
    pub fn new(x: f64) -> Self {
        Self {
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

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            ax.call_method("axvline", (self.x,), Some(&kwargs))?;
            Ok(())
        })
    }
}

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
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.axvline.html>
pub fn axvline(x: f64) -> PyAxvlineBuilder {
    PyAxvlineBuilder::new(x)
}
