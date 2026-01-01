use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

#[derive(KwargsBuilder)]
pub struct SavefigBuilder<'a> {
    figure: &'a Figure,
    fname: String,
    dpi: Option<f64>,
    format: Option<String>,
    transparent: Option<bool>,
    bbox_inches: Option<String>,
    pad_inches: Option<f64>,
    facecolor: Option<String>,
    edgecolor: Option<String>,
}

impl<'a> SavefigBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            figure_obj.call_method("savefig", (self.fname,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
    /// Save the current figure to a file.
    ///
    /// # Arguments
    /// * `fname` - A file path or filename (required). Format is inferred from the extension.
    ///
    /// Optional parameters via builder pattern:
    /// * `dpi` - Resolution in dots per inch (default: figure's dpi)
    /// * `format` - File format (e.g., "png", "pdf", "svg"). If not specified, inferred from fname extension.
    /// * `transparent` - If true, figure and axes patches are transparent
    /// * `bbox_inches` - Bounding box in inches. Use "tight" to fit the figure tightly.
    /// * `pad_inches` - Padding around figure when bbox_inches is "tight" (default: 0.1)
    /// * `facecolor` - Figure facecolor (use "auto" for current color)
    /// * `edgecolor` - Figure edgecolor (use "auto" for current color)
    ///
    /// # Examples
    /// ```ignore
    /// // Simple save with inferred format
    /// fig.savefig("plot.png".to_string())
    ///     .set()?;
    ///
    /// // High-resolution PNG with tight bounding box
    /// fig.savefig("plot.png".to_string())
    ///     .dpi(300.0)
    ///     .bbox_inches("tight".to_string())
    ///     .set()?;
    ///
    /// // PDF with transparency
    /// fig.savefig("output.pdf".to_string())
    ///     .format("pdf".to_string())
    ///     .transparent(true)
    ///     .set()?;
    ///
    /// // Custom styling
    /// fig.savefig("styled.png".to_string())
    ///     .dpi(150.0)
    ///     .facecolor("white".to_string())
    ///     .edgecolor("black".to_string())
    ///     .bbox_inches("tight".to_string())
    ///     .pad_inches(0.2)
    ///     .set()?;
    /// ```
    pub fn savefig(&self, fname: String) -> SavefigBuilder<'_> {
        SavefigBuilder {
            figure: self,
            fname,
            dpi: None,
            format: None,
            transparent: None,
            bbox_inches: None,
            pad_inches: None,
            facecolor: None,
            edgecolor: None,
        }
    }
}
