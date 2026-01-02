use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::shared::get_plt;

// Enum for bins parameter
pub enum HistBins {
    Count(i32),
    Edges(Array1<f64>),
    Strategy(String),
}

impl From<i32> for HistBins {
    fn from(value: i32) -> Self {
        HistBins::Count(value)
    }
}

impl From<Array1<f64>> for HistBins {
    fn from(value: Array1<f64>) -> Self {
        HistBins::Edges(value)
    }
}

impl From<&str> for HistBins {
    fn from(value: &str) -> Self {
        HistBins::Strategy(value.to_string())
    }
}

impl From<String> for HistBins {
    fn from(value: String) -> Self {
        HistBins::Strategy(value)
    }
}

#[derive(KwargsBuilder)]
pub struct PyHistBuilder {
    x: Array1<f64>,
    #[kwargs_builder(skip)]
    bins: Option<HistBins>,
    #[kwargs_builder(skip)]
    range: Option<(f64, f64)>,
    density: Option<bool>,
    cumulative: Option<bool>,
    histtype: Option<String>,
    orientation: Option<String>,
    rwidth: Option<f64>,
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    label: Option<String>,
}

impl PyHistBuilder {
    pub fn new(x: Array1<f64>) -> Self {
        Self {
            x,
            bins: None,
            range: None,
            density: None,
            cumulative: None,
            histtype: None,
            orientation: None,
            rwidth: None,
            color: None,
            edgecolor: None,
            linewidth: None,
            alpha: None,
            label: None,
        }
    }

    pub fn bins<T: Into<HistBins>>(mut self, value: T) -> Self {
        self.bins = Some(value.into());
        self
    }

    pub fn range(mut self, value: (f64, f64)) -> Self {
        self.range = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            let x_numpy = self.x.into_pyarray(py);

            if let Some(bins_value) = self.bins {
                match bins_value {
                    HistBins::Count(val) => kwargs.set_item("bins", val)?,
                    HistBins::Edges(arr) => kwargs.set_item("bins", arr.into_pyarray(py))?,
                    HistBins::Strategy(strategy) => kwargs.set_item("bins", strategy)?,
                }
            }

            if let Some((lower, upper)) = self.range {
                kwargs.set_item("range", (lower, upper))?;
            }

            ax.call_method("hist", (x_numpy,), Some(&kwargs))?;
            Ok(())
        })
    }
}

pub fn hist(x: Array1<f64>) -> PyHistBuilder {
    PyHistBuilder::new(x)
}
