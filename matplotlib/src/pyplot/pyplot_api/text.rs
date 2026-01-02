use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyTextBuilder {
    x: f64,
    y: f64,
    s: String,
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    horizontalalignment: Option<String>,
    verticalalignment: Option<String>,
    color: Option<String>,
    alpha: Option<f64>,
    backgroundcolor: Option<String>,
    rotation: Option<f64>,
    label: Option<String>,
    zorder: Option<i32>,
}

impl PyTextBuilder {
    pub fn new(x: f64, y: f64, s: String) -> Self {
        Self {
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

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            ax.call_method("text", (self.x, self.y, self.s), Some(&kwargs))?;
            Ok(())
        })
    }
}

pub fn text(x: f64, y: f64, s: String) -> PyTextBuilder {
    PyTextBuilder::new(x, y, s)
}
