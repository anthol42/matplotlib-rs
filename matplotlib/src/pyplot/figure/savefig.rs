use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

#[derive(KwargsBuilder)]
pub struct SavefigBuilder<'a> {
    figure: &'a Figure,
    fname: String,
    dpi: Option<f64>,
    format: Option<String>,
    transparent: Option<bool>,
    bbox_inches: Option<String>,
    pad_inches: Option<f64>,
    facecolor: Option<String>,
    edgecolor: Option<String>,
}

impl<'a> SavefigBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            figure_obj.call_method("savefig", (self.fname,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
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
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.figure.Figure.savefig.html>
    pub fn savefig(&self, fname: String) -> SavefigBuilder<'_> {
        SavefigBuilder {
            figure: self,
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
}
