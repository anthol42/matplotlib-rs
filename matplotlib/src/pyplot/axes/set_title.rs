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
    /// # Arguments
    /// * `label` - The title text (required)
    ///
    /// Optional parameters via builder pattern:
    /// * `loc` - Title position (TitleLoc::Center, Left, or Right)
    /// * `y` - Vertical Axes location for the title (1.0 is the top)
    /// * `pad` - Offset of the title from top of Axes in points
    /// * `fontsize` - Font size in points
    /// * `fontweight` - Font weight (e.g., "bold", "normal")
    /// * `fontstyle` - Font style (e.g., "italic", "normal")
    /// * `fontfamily` - Font family (e.g., "serif", "monospace")
    /// * `color` - Text color
    /// * `rotation` - Rotation angle in degrees
    /// * `alpha` - Transparency (0.0 to 1.0)
    /// * `backgroundcolor` - Background color for the title
    ///
    /// # Examples
    /// ```ignore
    /// // Simple title
    /// axes.set_title("My Plot".to_string())
    ///     .set()?;
    ///
    /// // Styled title
    /// axes.set_title("Temperature vs Time".to_string())
    ///     .fontsize(16.0)
    ///     .fontweight("bold".to_string())
    ///     .color("darkblue".to_string())
    ///     .set()?;
    ///
    /// // Positioned title with custom padding
    /// axes.set_title("Left Title".to_string())
    ///     .loc(TitleLoc::Left)
    ///     .fontsize(14.0)
    ///     .pad(20.0)
    ///     .set()?;
    /// ```
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
