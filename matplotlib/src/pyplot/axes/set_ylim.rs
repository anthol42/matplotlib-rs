use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct SetYlimBuilder<'a> {
    axes: &'a Axes,
    bottom: Option<f64>,
    top: Option<f64>,
    auto: Option<bool>,
}

impl<'a> SetYlimBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            axes_obj.call_method("set_ylim", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Set the y-axis view limits.
    ///
    /// # Parameters
    /// - `bottom`: Option<f64>
    /// - `top`: Option<f64>
    /// - `auto`: Option<bool>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_ylim.html>
    pub fn set_ylim(&self) -> SetYlimBuilder<'_> {
        SetYlimBuilder {
            axes: self,
            bottom: None,
            top: None,
            auto: None,
        }
    }
}
