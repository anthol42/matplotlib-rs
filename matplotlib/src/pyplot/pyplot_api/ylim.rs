use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyYlimBuilder {
    bottom: Option<f64>,
    top: Option<f64>,
    auto: Option<bool>,
}

impl PyYlimBuilder {
    pub fn new() -> Self {
        Self {
            bottom: None,
            top: None,
            auto: None,
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let kwargs = self.get_kwargs(py)?;

            plt.call_method("ylim", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

/// Get or set the y limits of the current axes.
///
/// # Examples
/// ```ignore
/// plt::ylim()
///     .bottom(0.0)
///     .top(100.0)
///     .set()?;
/// ```
pub fn ylim() -> PyYlimBuilder {
    PyYlimBuilder::new()
}
