use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct TextBuilder<'a> {
    axes: &'a Axes,
    x: f64,
    y: f64,
    s: String,
    // Font properties
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    // Alignment
    horizontalalignment: Option<String>,
    verticalalignment: Option<String>,
    // Styling
    color: Option<String>,
    alpha: Option<f64>,
    backgroundcolor: Option<String>,
    rotation: Option<f64>,
    // Other
    label: Option<String>,
    zorder: Option<i32>,
}

impl<'a> TextBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            axes_obj.call_method("text", (self.x, self.y, self.s), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    pub fn text(&self, x: f64, y: f64, s: String) -> TextBuilder<'_> {
        TextBuilder {
            axes: self,
            x,
            y,
            s,
            fontsize: None,
            fontweight: None,
            fontstyle: None,
            fontfamily: None,
            horizontalalignment: None,
            verticalalignment: None,
            color: None,
            alpha: None,
            backgroundcolor: None,
            rotation: None,
            label: None,
            zorder: None,
        }
    }
}
