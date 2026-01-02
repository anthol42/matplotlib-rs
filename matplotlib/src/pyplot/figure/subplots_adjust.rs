use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

#[derive(KwargsBuilder)]
pub struct SubplotsAdjustBuilder<'a> {
    figure: &'a Figure,
    left: Option<f64>,
    bottom: Option<f64>,
    right: Option<f64>,
    top: Option<f64>,
    wspace: Option<f64>,
    hspace: Option<f64>,
}

impl<'a> SubplotsAdjustBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            figure_obj.call_method("subplots_adjust", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
    /// Adjust the subplot layout parameters.
    ///
    /// All parameters are optional and specified as fractions of the figure dimensions (0.0 to 1.0).
    /// Unset parameters are left unmodified.
    ///
    /// # Optional Parameters via Builder Pattern
    ///
    /// **Subplot Area Margins:**
    /// * `left` - Position of the left edge of subplots (fraction of figure width)
    /// * `right` - Position of the right edge of subplots (fraction of figure width)
    /// * `bottom` - Position of the bottom edge of subplots (fraction of figure height)
    /// * `top` - Position of the top edge of subplots (fraction of figure height)
    ///
    /// **Spacing Between Subplots:**
    /// * `wspace` - Width padding between subplots (fraction of average axes width)
    /// * `hspace` - Height padding between subplots (fraction of average axes height)
    ///
    /// # Examples
    /// ```ignore
    /// // Adjust margins to leave space for labels
    /// fig.subplots_adjust()
    ///     .left(0.1)
    ///     .right(0.9)
    ///     .bottom(0.1)
    ///     .top(0.9)
    ///     .set()?;
    ///
    /// // Adjust spacing between subplots
    /// fig.subplots_adjust()
    ///     .wspace(0.3)
    ///     .hspace(0.4)
    ///     .set()?;
    ///
    /// // Comprehensive adjustment
    /// fig.subplots_adjust()
    ///     .left(0.08)
    ///     .right(0.95)
    ///     .bottom(0.08)
    ///     .top(0.92)
    ///     .wspace(0.25)
    ///     .hspace(0.35)
    ///     .set()?;
    /// ```
    pub fn subplots_adjust(&self) -> SubplotsAdjustBuilder<'_> {
        SubplotsAdjustBuilder {
            figure: self,
            left: None,
            bottom: None,
            right: None,
            top: None,
            wspace: None,
            hspace: None,
        }
    }
}
