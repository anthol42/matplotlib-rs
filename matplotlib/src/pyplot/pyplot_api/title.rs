use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

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
pub struct PyTitleBuilder {
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

impl PyTitleBuilder {
    pub fn new(label: String) -> Self {
        Self {
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

    /// Set the title position (center, left, or right).
    pub fn loc(mut self, value: TitleLoc) -> Self {
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

            plt.call_method("title", (self.label,), Some(&kwargs))?;

            Ok(())
        })
    }
}

/// Set a title for the current axes.
///
/// # Examples
/// ```ignore
/// plt::title("My Plot".to_string())
///     .fontsize(14.0)
///     .fontweight("bold".to_string())
///     .set()?;
/// ```
pub fn title(label: String) -> PyTitleBuilder {
    PyTitleBuilder::new(label)
}
