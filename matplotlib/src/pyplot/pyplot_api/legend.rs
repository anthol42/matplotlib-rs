use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

/// Valid options for legend location.
#[derive(Debug, Clone, Copy)]
pub enum LegendLoc {
    /// Automatically choose the best location
    Best,
    /// Upper right corner
    UpperRight,
    /// Upper left corner
    UpperLeft,
    /// Lower left corner
    LowerLeft,
    /// Lower right corner
    LowerRight,
    /// Right side, centered
    Right,
    /// Center left
    CenterLeft,
    /// Center right
    CenterRight,
    /// Lower center
    LowerCenter,
    /// Upper center
    UpperCenter,
    /// Center
    Center,
}

impl LegendLoc {
    fn as_str(&self) -> &str {
        match self {
            LegendLoc::Best => "best",
            LegendLoc::UpperRight => "upper right",
            LegendLoc::UpperLeft => "upper left",
            LegendLoc::LowerLeft => "lower left",
            LegendLoc::LowerRight => "lower right",
            LegendLoc::Right => "right",
            LegendLoc::CenterLeft => "center left",
            LegendLoc::CenterRight => "center right",
            LegendLoc::LowerCenter => "lower center",
            LegendLoc::UpperCenter => "upper center",
            LegendLoc::Center => "center",
        }
    }
}

/// Valid options for legend text alignment.
#[derive(Debug, Clone, Copy)]
pub enum LegendAlignment {
    /// Center alignment
    Center,
    /// Left alignment
    Left,
    /// Right alignment
    Right,
}

impl LegendAlignment {
    fn as_str(&self) -> &str {
        match self {
            LegendAlignment::Center => "center",
            LegendAlignment::Left => "left",
            LegendAlignment::Right => "right",
        }
    }
}

#[derive(KwargsBuilder)]
pub struct PyLegendBuilder {
    // Location & Positioning
    #[kwargs_builder(skip)]
    loc: Option<LegendLoc>,
    #[kwargs_builder(skip)]
    bbox_to_anchor: Option<(f64, f64)>,
    // Text & Font
    fontsize: Option<f64>,
    labelcolor: Option<String>,
    title: Option<String>,
    title_fontsize: Option<f64>,
    #[kwargs_builder(skip)]
    alignment: Option<LegendAlignment>,
    // Layout & Spacing
    ncols: Option<i32>,
    labelspacing: Option<f64>,
    borderpad: Option<f64>,
    handlelength: Option<f64>,
    columnspacing: Option<f64>,
    // Markers
    markerscale: Option<f64>,
    // Appearance
    frameon: Option<bool>,
    fancybox: Option<bool>,
    shadow: Option<bool>,
    framealpha: Option<f64>,
    facecolor: Option<String>,
    edgecolor: Option<String>,
}

impl PyLegendBuilder {
    pub fn new() -> Self {
        Self {
            loc: None,
            bbox_to_anchor: None,
            fontsize: None,
            labelcolor: None,
            title: None,
            title_fontsize: None,
            alignment: None,
            ncols: None,
            labelspacing: None,
            borderpad: None,
            handlelength: None,
            columnspacing: None,
            markerscale: None,
            frameon: None,
            fancybox: None,
            shadow: None,
            framealpha: None,
            facecolor: None,
            edgecolor: None,
        }
    }

    /// Set the legend location.
    pub fn loc(mut self, value: LegendLoc) -> Self {
        self.loc = Some(value);
        self
    }

    /// Set the bounding box anchor for legend positioning.
    /// Accepts a 2-tuple (x, y) to position the legend.
    pub fn bbox_to_anchor(mut self, value: (f64, f64)) -> Self {
        self.bbox_to_anchor = Some(value);
        self
    }

    /// Set the legend text alignment.
    pub fn alignment(mut self, value: LegendAlignment) -> Self {
        self.alignment = Some(value);
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

            // Add bbox_to_anchor parameter if specified (tuple)
            if let Some((x, y)) = self.bbox_to_anchor {
                kwargs.set_item("bbox_to_anchor", (x, y))?;
            }

            // Add alignment parameter if specified
            if let Some(alignment_val) = self.alignment {
                kwargs.set_item("alignment", alignment_val.as_str())?;
            }

            plt.call_method("legend", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

/// Place a legend on the current axes.
///
/// # Examples
/// ```ignore
/// plt::legend()
///     .loc(LegendLoc::UpperRight)
///     .fontsize(10.0)
///     .set()?;
/// ```
pub fn legend() -> PyLegendBuilder {
    PyLegendBuilder::new()
}
