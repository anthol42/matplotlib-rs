use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Figure;

#[derive(KwargsBuilder)]
pub struct TightLayoutBuilder<'a> {
    figure: &'a Figure,
    pad: Option<f64>,
    h_pad: Option<f64>,
    w_pad: Option<f64>,
    #[kwargs_builder(skip)]
    rect: Option<(f64, f64, f64, f64)>,
}

impl<'a> TightLayoutBuilder<'a> {
    /// Set the rectangle (left, bottom, right, top) in normalized figure coordinates.
    pub fn rect(mut self, value: (f64, f64, f64, f64)) -> Self {
        self.rect = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let figure_obj = self.figure.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Add rect parameter if specified (tuple)
            if let Some((left, bottom, right, top)) = self.rect {
                kwargs.set_item("rect", (left, bottom, right, top))?;
            }

            figure_obj.call_method("tight_layout", (), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Figure {
    /// Adjust the padding between and around subplots.
    ///
    /// # Parameters
    /// - `pad`: Option<f64>
    /// - `h_pad`: Option<f64>
    /// - `w_pad`: Option<f64>
    /// - `rect`: Option<(f64, f64, f64, f64)> - (left, bottom, right, top)
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.figure.Figure.tight_layout.html>
    pub fn tight_layout(&self) -> TightLayoutBuilder<'_> {
        TightLayoutBuilder {
            figure: self,
            pad: None,
            h_pad: None,
            w_pad: None,
            rect: None,
        }
    }
}
