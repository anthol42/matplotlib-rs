use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

/// Valid options for x-axis label position.
#[derive(Debug, Clone, Copy)]
pub enum XLabelLoc {
    /// Position label at left of x-axis
    Left,
    /// Position label at center of x-axis
    Center,
    /// Position label at right of x-axis
    Right,
}

impl XLabelLoc {
    fn as_str(&self) -> &str {
        match self {
            XLabelLoc::Left => "left",
            XLabelLoc::Center => "center",
            XLabelLoc::Right => "right",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct SetXlabelBuilder<'a> {
    axes: &'a Axes,
    xlabel: String,
    labelpad: Option<f64>,
    #[kwargs_builder(skip)]
    loc: Option<XLabelLoc>,
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

impl<'a> SetXlabelBuilder<'a> {
    /// Set the label position (left, center, or right).
    pub fn loc(mut self, value: XLabelLoc) -> Self {
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

            axes_obj.call_method("set_xlabel", (self.xlabel,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Set the label for the x-axis.
    ///
    /// # Parameters
    /// - `xlabel`: String
    /// - `labelpad`: Option<f64>
    /// - `loc`: Option<XLabelLoc> - Left, Center, or Right
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
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_xlabel.html>
    pub fn set_xlabel(&self, xlabel: String) -> SetXlabelBuilder<'_> {
        SetXlabelBuilder {
            axes: self,
            xlabel,
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
