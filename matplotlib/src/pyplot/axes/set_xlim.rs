use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct SetXlimBuilder<'a> {
    axes: &'a Axes,
    left: Option<f64>,
    right: Option<f64>,
    auto: Option<bool>,
}

impl<'a> SetXlimBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            axes_obj.call_method("set_xlim", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Set the x-axis view limits.
    ///
    /// # Parameters
    /// - `left`: Option<f64>
    /// - `right`: Option<f64>
    /// - `auto`: Option<bool>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_xlim.html>
    pub fn set_xlim(&self) -> SetXlimBuilder<'_> {
        SetXlimBuilder {
            axes: self,
            left: None,
            right: None,
            auto: None,
        }
    }
}
