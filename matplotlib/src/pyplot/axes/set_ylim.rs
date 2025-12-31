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
    /// # Arguments
    /// All parameters are optional and can be set using the builder pattern:
    /// * `bottom` - The bottom ylim in data coordinates
    /// * `top` - The top ylim in data coordinates
    /// * `auto` - Whether to turn on autoscaling of the y-axis
    ///
    /// # Examples
    /// ```ignore
    /// // Set both limits
    /// axes.set_ylim()
    ///     .bottom(0.0)
    ///     .top(10.0)
    ///     .set()?;
    ///
    /// // Set only one limit (leave the other unchanged)
    /// axes.set_ylim()
    ///     .top(5.0)
    ///     .set()?;
    ///
    /// // Reverse limits to flip y-axis direction (e.g., for depth)
    /// axes.set_ylim()
    ///     .bottom(5000.0)
    ///     .top(0.0)
    ///     .set()?;
    ///
    /// // Enable autoscaling
    /// axes.set_ylim()
    ///     .auto(true)
    ///     .set()?;
    /// ```
    pub fn set_ylim(&self) -> SetYlimBuilder<'_> {
        SetYlimBuilder {
            axes: self,
            bottom: None,
            top: None,
            auto: None,
        }
    }
}
