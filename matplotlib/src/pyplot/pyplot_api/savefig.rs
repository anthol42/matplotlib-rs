use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PySavefigBuilder {
    fname: String,
    dpi: Option<f64>,
    format: Option<String>,
    transparent: Option<bool>,
    bbox_inches: Option<String>,
    pad_inches: Option<f64>,
    facecolor: Option<String>,
    edgecolor: Option<String>,
}

impl PySavefigBuilder {
    pub fn new(fname: String) -> Self {
        Self {
            fname,
            dpi: None,
            format: None,
            transparent: None,
            bbox_inches: None,
            pad_inches: None,
            facecolor: None,
            edgecolor: None,
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let fig = plt.call_method0("gcf")?;
            let kwargs = self.get_kwargs(py)?;

            fig.call_method("savefig", (self.fname,), Some(&kwargs))?;
            Ok(())
        })
    }
}

/// Save the current figure.
///
/// # Parameters
/// - `fname`: String
/// - `dpi`: Option<f64>
/// - `format`: Option<String>
/// - `transparent`: Option<bool>
/// - `bbox_inches`: Option<String>
/// - `pad_inches`: Option<f64>
/// - `facecolor`: Option<String>
/// - `edgecolor`: Option<String>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.savefig.html>
pub fn savefig(fname: String) -> PySavefigBuilder {
    PySavefigBuilder::new(fname)
}
