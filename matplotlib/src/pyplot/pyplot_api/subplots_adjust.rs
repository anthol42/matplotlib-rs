use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PySubplotsAdjustBuilder {
    left: Option<f64>,
    bottom: Option<f64>,
    right: Option<f64>,
    top: Option<f64>,
    wspace: Option<f64>,
    hspace: Option<f64>,
}

impl PySubplotsAdjustBuilder {
    pub fn new() -> Self {
        Self {
            left: None,
            bottom: None,
            right: None,
            top: None,
            wspace: None,
            hspace: None,
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let fig = plt.call_method0("gcf")?;
            let kwargs = self.get_kwargs(py)?;

            fig.call_method("subplots_adjust", (), Some(&kwargs))?;
            Ok(())
        })
    }
}

pub fn subplots_adjust() -> PySubplotsAdjustBuilder {
    PySubplotsAdjustBuilder::new()
}
