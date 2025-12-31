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
    /// # Arguments
    /// All parameters are optional and can be set using the builder pattern:
    /// * `left` - The left xlim in data coordinates
    /// * `right` - The right xlim in data coordinates
    /// * `auto` - Whether to turn on autoscaling of the x-axis
    ///
    /// # Examples
    /// ```ignore
    /// // Set both limits
    /// axes.set_xlim()
    ///     .left(0.0)
    ///     .right(10.0)
    ///     .set()?;
    ///
    /// // Set only one limit (leave the other unchanged)
    /// axes.set_xlim()
    ///     .right(5.0)
    ///     .set()?;
    ///
    /// // Reverse limits to flip x-axis direction
    /// axes.set_xlim()
    ///     .left(10.0)
    ///     .right(0.0)
    ///     .set()?;
    ///
    /// // Enable autoscaling
    /// axes.set_xlim()
    ///     .auto(true)
    ///     .set()?;
    /// ```
    pub fn set_xlim(&self) -> SetXlimBuilder<'_> {
        SetXlimBuilder {
            axes: self,
            left: None,
            right: None,
            auto: None,
        }
    }
}
