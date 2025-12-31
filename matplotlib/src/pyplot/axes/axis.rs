use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;

use super::Axes;

/// Valid axis options for compile-time validation.
///
/// These options control the appearance and scaling of axes.
#[derive(Debug, Clone, Copy)]
pub enum AxisOption {
    /// Show axis decorations (labels, spines, ticks, grid)
    On,
    /// Hide all axis decorations
    Off,
    /// Equal scaling (circles appear circular) by changing axis limits
    Equal,
    /// Equal scaling by changing plot box dimensions
    Scaled,
    /// Set limits to show all data, then disable autoscaling
    Tight,
    /// Automatic scaling (fill plot box with data)
    Auto,
    /// 'Scaled' with axis limits equal to data limits
    Image,
    /// Square plot (force xmax-xmin == ymax-ymin)
    Square,
}

impl AxisOption {
    /// Convert the enum variant to the string expected by matplotlib
    fn as_str(&self) -> &str {
        match self {
            AxisOption::On => "on",
            AxisOption::Off => "off",
            AxisOption::Equal => "equal",
            AxisOption::Scaled => "scaled",
            AxisOption::Tight => "tight",
            AxisOption::Auto => "auto",
            AxisOption::Image => "image",
            AxisOption::Square => "square",
        }
    }
}

pub struct AxisBuilder<'a> {
    axes: &'a Axes,
    option: AxisOption,
}

impl<'a> AxisBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            axes_obj.call_method("axis", (self.option.as_str(),), None)?;
            Ok(())
        })
    }
}

impl Axes {
    /// Set axis properties using predefined options.
    ///
    /// # Arguments
    /// * `option` - An AxisOption enum variant (compile-time validated)
    ///
    /// # Examples
    /// ```ignore
    /// // Equal aspect ratio (circles appear circular)
    /// axes.axis(AxisOption::Equal).set()?;
    ///
    /// // Hide all axis decorations
    /// axes.axis(AxisOption::Off).set()?;
    ///
    /// // Tight limits showing all data
    /// axes.axis(AxisOption::Tight).set()?;
    /// ```
    pub fn axis(&self, option: AxisOption) -> AxisBuilder<'_> {
        AxisBuilder {
            axes: self,
            option,
        }
    }
}
