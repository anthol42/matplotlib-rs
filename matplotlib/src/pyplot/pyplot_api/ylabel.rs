use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

/// Valid options for y-axis label position.
#[derive(Debug, Clone, Copy)]
pub enum YLabelLoc {
    /// Position label at bottom of y-axis
    Bottom,
    /// Position label at center of y-axis
    Center,
    /// Position label at top of y-axis
    Top,
}

impl YLabelLoc {
    fn as_str(&self) -> &str {
        match self {
            YLabelLoc::Bottom => "bottom",
            YLabelLoc::Center => "center",
            YLabelLoc::Top => "top",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct PyYlabelBuilder {
    ylabel: String,
    labelpad: Option<f64>,
    #[kwargs_builder(skip)]
    loc: Option<YLabelLoc>,
    // Common text properties
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    color: Option<String>,
    rotation: Option<f64>,
    alpha: Option<f64>,
    backgroundcolor: Option<String>,
}

impl PyYlabelBuilder {
    pub fn new(ylabel: String) -> Self {
        Self {
            ylabel,
            labelpad: None,
            loc: None,
            fontsize: None,
            fontweight: None,
            fontstyle: None,
            fontfamily: None,
            color: None,
            rotation: None,
            alpha: None,
            backgroundcolor: None,
        }
    }

    /// Set the label position (bottom, center, or top).
    pub fn loc(mut self, value: YLabelLoc) -> Self {
        self.loc = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Add loc parameter if specified
            if let Some(loc_val) = self.loc {
                kwargs.set_item("loc", loc_val.as_str())?;
            }

            plt.call_method("ylabel", (self.ylabel,), Some(&kwargs))?;

            Ok(())
        })
    }
}

/// Set the label for the y-axis.
///
/// # Examples
/// ```ignore
/// plt::ylabel("Voltage (V)".to_string())
///     .fontsize(12.0)
///     .set()?;
/// ```
pub fn ylabel(label: String) -> PyYlabelBuilder {
    PyYlabelBuilder::new(label)
}
