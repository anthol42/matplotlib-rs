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
    /// Add a centered super title to the figure.
    ///
    /// The suptitle appears at the top of the figure, spanning all subplots.
    ///
    /// # Arguments
    /// * `t` - The super title text (required)
    ///
    /// Optional parameters via builder pattern:
    ///
    /// **Positioning:**
    /// * `x` - X location in figure coordinates (default: 0.5)
    /// * `y` - Y location in figure coordinates (default: 0.98)
    /// * `horizontalalignment` - Horizontal alignment (SuptitleHAlign::Center, Left, or Right)
    /// * `verticalalignment` - Vertical alignment (SuptitleVAlign::Top, Center, Bottom, or Baseline)
    ///
    /// **Text Properties:**
    /// * `fontsize` - Font size in points
    /// * `fontweight` - Font weight (e.g., "bold", "normal")
    /// * `fontstyle` - Font style (e.g., "italic", "normal")
    /// * `fontfamily` - Font family (e.g., "serif", "monospace")
    /// * `color` - Text color
    /// * `alpha` - Transparency (0.0 to 1.0)
    /// * `backgroundcolor` - Background color for the title
    ///
    /// # Examples
    /// ```ignore
    /// // Simple suptitle
    /// fig.suptitle("Main Title".to_string())
    ///     .set()?;
    ///
    /// // Styled suptitle
    /// fig.suptitle("Experiment Results".to_string())
    ///     .fontsize(16.0)
    ///     .fontweight("bold".to_string())
    ///     .color("darkblue".to_string())
    ///     .set()?;
    ///
    /// // Positioned suptitle
    /// fig.suptitle("Custom Position".to_string())
    ///     .x(0.5)
    ///     .y(0.95)
    ///     .horizontalalignment(SuptitleHAlign::Center)
    ///     .fontsize(14.0)
    ///     .set()?;
    /// ```
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
