use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

/// Valid options for which grid lines to show.
#[derive(Debug, Clone, Copy)]
pub enum GridWhich {
    /// Major grid lines only
    Major,
    /// Minor grid lines only
    Minor,
    /// Both major and minor grid lines
    Both,
}

impl GridWhich {
    fn as_str(&self) -> &str {
        match self {
            GridWhich::Major => "major",
            GridWhich::Minor => "minor",
            GridWhich::Both => "both",
        }
    }
}

/// Valid options for which axis to apply grid to.
#[derive(Debug, Clone, Copy)]
pub enum GridAxis {
    /// Apply to both x and y axes
    Both,
    /// Apply to x-axis only
    X,
    /// Apply to y-axis only
    Y,
}

impl GridAxis {
    fn as_str(&self) -> &str {
        match self {
            GridAxis::Both => "both",
            GridAxis::X => "x",
            GridAxis::Y => "y",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct GridBuilder<'a> {
    axes: &'a Axes,
    visible: Option<bool>,
    #[kwargs_builder(skip)]
    which: Option<GridWhich>,
    #[kwargs_builder(skip)]
    axis: Option<GridAxis>,
    color: Option<String>,
    linestyle: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
}

impl<'a> GridBuilder<'a> {
    /// Set which grid lines to show (major, minor, or both).
    pub fn which(mut self, value: GridWhich) -> Self {
        self.which = Some(value);
        self
    }

    /// Set which axis to apply grid to (both, x, or y).
    pub fn axis(mut self, value: GridAxis) -> Self {
        self.axis = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Add which parameter if specified
            if let Some(which_val) = self.which {
                kwargs.set_item("which", which_val.as_str())?;
            }

            // Add axis parameter if specified
            if let Some(axis_val) = self.axis {
                kwargs.set_item("axis", axis_val.as_str())?;
            }

            axes_obj.call_method("grid", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Configure the grid lines.
    ///
    /// By default, the grid is made visible when this method is called.
    ///
    /// # Arguments
    /// All parameters are optional and can be set using the builder pattern:
    /// * `visible` - Whether to show the grid (default: true)
    /// * `which` - Which grid lines to show (GridWhich::Major, Minor, or Both)
    /// * `axis` - Which axis to apply to (GridAxis::Both, X, or Y)
    /// * `color` - Grid line color
    /// * `linestyle` - Line style ("-", "--", "-.", ":")
    /// * `linewidth` - Line width in points
    /// * `alpha` - Transparency (0.0 to 1.0)
    ///
    /// # Examples
    /// ```ignore
    /// // Show major grid lines with default styling
    /// axes.grid().set()?;
    ///
    /// // Customize grid appearance
    /// axes.grid()
    ///     .color("gray".to_string())
    ///     .linestyle("--".to_string())
    ///     .alpha(0.5)
    ///     .set()?;
    ///
    /// // Show both major and minor grid lines on x-axis only
    /// axes.grid()
    ///     .which(GridWhich::Both)
    ///     .axis(GridAxis::X)
    ///     .set()?;
    ///
    /// // Hide the grid
    /// axes.grid()
    ///     .visible(false)
    ///     .set()?;
    /// ```
    pub fn grid(&self) -> GridBuilder<'_> {
        GridBuilder {
            axes: self,
            visible: Some(true),  // Default to true when grid() is called
            which: None,
            axis: None,
            color: None,
            linestyle: None,
            linewidth: None,
            alpha: None,
        }
    }
}
