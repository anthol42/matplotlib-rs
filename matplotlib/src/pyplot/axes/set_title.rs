use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

/// Valid options for title position.
#[derive(Debug, Clone, Copy)]
pub enum TitleLoc {
    /// Position title at center
    Center,
    /// Position title at left
    Left,
    /// Position title at right
    Right,
}

impl TitleLoc {
    fn as_str(&self) -> &str {
        match self {
            TitleLoc::Center => "center",
            TitleLoc::Left => "left",
            TitleLoc::Right => "right",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct SetTitleBuilder<'a> {
    axes: &'a Axes,
    label: String,
    #[kwargs_builder(skip)]
    loc: Option<TitleLoc>,
    y: Option<f64>,
    pad: Option<f64>,
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

impl<'a> SetTitleBuilder<'a> {
    /// Set the title position (center, left, or right).
    pub fn loc(mut self, value: TitleLoc) -> Self {
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

            axes_obj.call_method("set_title", (self.label,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Set a title for the Axes.
    ///
    /// # Parameters
    /// - `label`: String
    /// - `loc`: Option<TitleLoc> - Center, Left, or Right
    /// - `y`: Option<f64>
    /// - `pad`: Option<f64>
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
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_title.html>
    pub fn set_title(&self, label: String) -> SetTitleBuilder<'_> {
        SetTitleBuilder {
            axes: self,
            label,
            loc: None,
            y: None,
            pad: None,
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
