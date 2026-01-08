use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(Debug, Clone, Copy)]
pub enum SuptitleHAlign {
    Center,
    Left,
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

#[derive(Debug, Clone, Copy)]
pub enum SuptitleVAlign {
    Top,
    Center,
    Bottom,
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
pub struct PySuptitleBuilder {
    t: String,
    x: Option<f64>,
    y: Option<f64>,
    #[kwargs_builder(skip)]
    horizontalalignment: Option<SuptitleHAlign>,
    #[kwargs_builder(skip)]
    verticalalignment: Option<SuptitleVAlign>,
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    color: Option<String>,
    alpha: Option<f64>,
    backgroundcolor: Option<String>,
}

impl PySuptitleBuilder {
    pub fn new(t: String) -> Self {
        Self {
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

    pub fn horizontalalignment(mut self, value: SuptitleHAlign) -> Self {
        self.horizontalalignment = Some(value);
        self
    }

    pub fn verticalalignment(mut self, value: SuptitleVAlign) -> Self {
        self.verticalalignment = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let fig = plt.call_method0("gcf")?;
            let kwargs = self.get_kwargs(py)?;

            if let Some(ha_val) = self.horizontalalignment {
                kwargs.set_item("horizontalalignment", ha_val.as_str())?;
            }

            if let Some(va_val) = self.verticalalignment {
                kwargs.set_item("verticalalignment", va_val.as_str())?;
            }

            fig.call_method("suptitle", (self.t,), Some(&kwargs))?;
            Ok(())
        })
    }
}

/// Add a centered suptitle to the figure.
///
/// # Parameters
/// - `t`: String
/// - `x`: Option<f64>
/// - `y`: Option<f64>
/// - `horizontalalignment`: Option<SuptitleHAlign> - Center, Left, or Right
/// - `verticalalignment`: Option<SuptitleVAlign> - Top, Center, Bottom, or Baseline
/// - `fontsize`: Option<f64>
/// - `fontweight`: Option<String>
/// - `fontstyle`: Option<String>
/// - `fontfamily`: Option<String>
/// - `color`: Option<String>
/// - `alpha`: Option<f64>
/// - `backgroundcolor`: Option<String>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.suptitle.html>
pub fn suptitle(t: String) -> PySuptitleBuilder {
    PySuptitleBuilder::new(t)
}
