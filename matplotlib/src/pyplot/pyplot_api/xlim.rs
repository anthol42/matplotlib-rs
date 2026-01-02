use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyXlimBuilder {
    left: Option<f64>,
    right: Option<f64>,
    auto: Option<bool>,
}

impl PyXlimBuilder {
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            auto: None,
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let kwargs = self.get_kwargs(py)?;

            plt.call_method("xlim", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

/// Get or set the x limits of the current axes.
///
/// # Examples
/// ```ignore
/// plt::xlim()
///     .left(0.0)
///     .right(10.0)
///     .set()?;
/// ```
pub fn xlim() -> PyXlimBuilder {
    PyXlimBuilder::new()
}
