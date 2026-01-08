use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

#[derive(KwargsBuilder)]
pub struct SubplotsAdjustBuilder<'a> {
    figure: &'a Figure,
    left: Option<f64>,
    bottom: Option<f64>,
    right: Option<f64>,
    top: Option<f64>,
    wspace: Option<f64>,
    hspace: Option<f64>,
}

impl<'a> SubplotsAdjustBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            figure_obj.call_method("subplots_adjust", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
    /// Adjust the subplot layout parameters.
    ///
    /// # Parameters
    /// - `left`: Option<f64>
    /// - `bottom`: Option<f64>
    /// - `right`: Option<f64>
    /// - `top`: Option<f64>
    /// - `wspace`: Option<f64>
    /// - `hspace`: Option<f64>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.figure.Figure.subplots_adjust.html>
    pub fn subplots_adjust(&self) -> SubplotsAdjustBuilder<'_> {
        SubplotsAdjustBuilder {
            figure: self,
            left: None,
            bottom: None,
            right: None,
            top: None,
            wspace: None,
            hspace: None,
        }
    }
}
