use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;

use super::shared::get_plt;

#[derive(Debug, Clone, Copy)]
pub enum AxisOption {
    On,
    Off,
    Equal,
    Scaled,
    Tight,
    Auto,
    Image,
    Square,
}

impl AxisOption {
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

pub struct PyAxisBuilder {
    option: AxisOption,
}

impl PyAxisBuilder {
    pub fn new(option: AxisOption) -> Self {
        Self { option }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            ax.call_method("axis", (self.option.as_str(),), None)?;
            Ok(())
        })
    }
}

/// Convenience method to get or set some axis properties.
///
/// # Parameters
/// - `option`: AxisOption - On, Off, Equal, Scaled, Tight, Auto, Image, or Square
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.axis.html>
pub fn axis(option: AxisOption) -> PyAxisBuilder {
    PyAxisBuilder::new(option)
}
