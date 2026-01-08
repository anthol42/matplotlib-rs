use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

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
pub struct SetYlabelBuilder<'a> {
    axes: &'a Axes,
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

impl<'a> SetYlabelBuilder<'a> {
    /// Set the label position (bottom, center, or top).
    pub fn loc(mut self, value: YLabelLoc) -> Self {
        self.loc = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Add loc parameter if specified
            if let Some(loc_val) = self.loc {
                kwargs.set_item("loc", loc_val.as_str())?;
            }

            axes_obj.call_method("set_ylabel", (self.ylabel,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Set the label for the y-axis.
    ///
    /// # Parameters
    /// - `ylabel`: String
    /// - `labelpad`: Option<f64>
    /// - `loc`: Option<YLabelLoc> - Bottom, Center, or Top
    /// - `fontsize`: Option<f64>
    /// - `fontweight`: Option<String>
    /// - `fontstyle`: Option<String>
    /// - `fontfamily`: Option<String>
    /// - `color`: Option<String>
    /// - `rotation`: Option<f64>
    /// - `alpha`: Option<f64>
    /// - `backgroundcolor`: Option<String>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_ylabel.html>
    pub fn set_ylabel(&self, ylabel: String) -> SetYlabelBuilder<'_> {
        SetYlabelBuilder {
            axes: self,
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
}
