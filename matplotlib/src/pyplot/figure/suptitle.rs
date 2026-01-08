use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

/// Valid options for horizontal alignment of suptitle.
#[derive(Debug, Clone, Copy)]
pub enum SuptitleHAlign {
    /// Center alignment
    Center,
    /// Left alignment
    Left,
    /// Right alignment
    Right,
}

impl SuptitleHAlign {
    fn as_str(&self) -> &str {
        match self {
            SuptitleHAlign::Center => "center",
            SuptitleHAlign::Left => "left",
            SuptitleHAlign::Right => "right",
        }
    }
}

/// Valid options for vertical alignment of suptitle.
#[derive(Debug, Clone, Copy)]
pub enum SuptitleVAlign {
    /// Top alignment
    Top,
    /// Center alignment
    Center,
    /// Bottom alignment
    Bottom,
    /// Baseline alignment
    Baseline,
}

impl SuptitleVAlign {
    fn as_str(&self) -> &str {
        match self {
            SuptitleVAlign::Top => "top",
            SuptitleVAlign::Center => "center",
            SuptitleVAlign::Bottom => "bottom",
            SuptitleVAlign::Baseline => "baseline",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct SuptitleBuilder<'a> {
    figure: &'a Figure,
    t: String,
    // Positioning
    x: Option<f64>,
    y: Option<f64>,
    #[kwargs_builder(skip)]
    horizontalalignment: Option<SuptitleHAlign>,
    #[kwargs_builder(skip)]
    verticalalignment: Option<SuptitleVAlign>,
    // Common text properties
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    color: Option<String>,
    alpha: Option<f64>,
    backgroundcolor: Option<String>,
}

impl<'a> SuptitleBuilder<'a> {
    /// Set the horizontal alignment.
    pub fn horizontalalignment(mut self, value: SuptitleHAlign) -> Self {
        self.horizontalalignment = Some(value);
        self
    }

    /// Set the vertical alignment.
    pub fn verticalalignment(mut self, value: SuptitleVAlign) -> Self {
        self.verticalalignment = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Add horizontalalignment parameter if specified
            if let Some(ha_val) = self.horizontalalignment {
                kwargs.set_item("horizontalalignment", ha_val.as_str())?;
            }

            // Add verticalalignment parameter if specified
            if let Some(va_val) = self.verticalalignment {
                kwargs.set_item("verticalalignment", va_val.as_str())?;
            }

            figure_obj.call_method("suptitle", (self.t,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
    /// Add a centered suptitle to the figure.
    ///
    /// # Parameters
    /// - `t`: String
    /// - `x`: Option<f64>
    /// - `y`: Option<f64>
    /// - `horizontalalignment`: Option<SuptitleHAlign> - Center, Left, or Right
    /// - `verticalalignment`: Option<SuptitleVAlign> - Top, Center, Bottom, or Baseline
    /// - `fontsize`: Option<f64>
    /// - `fontweight`: Option<String>
    /// - `fontstyle`: Option<String>
    /// - `fontfamily`: Option<String>
    /// - `color`: Option<String>
    /// - `alpha`: Option<f64>
    /// - `backgroundcolor`: Option<String>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.figure.Figure.suptitle.html>
    pub fn suptitle(&self, t: String) -> SuptitleBuilder<'_> {
        SuptitleBuilder {
            figure: self,
            t,
            x: None,
            y: None,
            horizontalalignment: None,
            verticalalignment: None,
            fontsize: None,
            fontweight: None,
            fontstyle: None,
            fontfamily: None,
            color: None,
            alpha: None,
            backgroundcolor: None,
        }
    }
}
