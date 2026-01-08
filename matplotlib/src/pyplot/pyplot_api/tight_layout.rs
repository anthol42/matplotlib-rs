use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyTightLayoutBuilder {
    pad: Option<f64>,
    h_pad: Option<f64>,
    w_pad: Option<f64>,
    #[kwargs_builder(skip)]
    rect: Option<(f64, f64, f64, f64)>,
}

impl PyTightLayoutBuilder {
    pub fn new() -> Self {
        Self {
            pad: None,
            h_pad: None,
            w_pad: None,
            rect: None,
        }
    }

    pub fn rect(mut self, value: (f64, f64, f64, f64)) -> Self {
        self.rect = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let fig = plt.call_method0("gcf")?;
            let kwargs = self.get_kwargs(py)?;

            if let Some((left, bottom, right, top)) = self.rect {
                kwargs.set_item("rect", (left, bottom, right, top))?;
            }

            fig.call_method("tight_layout", (), Some(&kwargs))?;
            Ok(())
        })
    }
}

/// Adjust the padding between and around subplots.
///
/// # Parameters
/// - `pad`: Option<f64>
/// - `h_pad`: Option<f64>
/// - `w_pad`: Option<f64>
/// - `rect`: Option<(f64, f64, f64, f64)> - (left, bottom, right, top)
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.tight_layout.html>
pub fn tight_layout() -> PyTightLayoutBuilder {
    PyTightLayoutBuilder::new()
}
