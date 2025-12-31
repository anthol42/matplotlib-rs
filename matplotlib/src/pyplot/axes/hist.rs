use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

// Enum for bins parameter (can be count, edges array, or strategy string)
pub enum HistBins {
    Count(i32),
    Edges(Array1<f64>),
    Strategy(String),
}

// Implement From traits for automatic type conversion - HistBins
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
pub struct HistBuilder<'a> {
    axes: &'a Axes,
    x: Array1<f64>,
    // Multi-type parameter (handled manually with custom setter)
    #[kwargs_builder(skip)]
    bins: Option<HistBins>,
    // Histogram behavior (range handled manually as tuple)
    #[kwargs_builder(skip)]
    range: Option<(f64, f64)>,
    density: Option<bool>,
    cumulative: Option<bool>,
    // Visualization style
    histtype: Option<String>,
    orientation: Option<String>,
    rwidth: Option<f64>,
    // Styling
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    label: Option<String>,
}

impl<'a> HistBuilder<'a> {
    // Manual method for bins with generic type conversion
    pub fn bins<T: Into<HistBins>>(mut self, value: T) -> Self {
        self.bins = Some(value.into());
        self
    }

    // Manual method for range (tuple)
    pub fn range(mut self, value: (f64, f64)) -> Self {
        self.range = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert Array1 to NumPy array (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);

            // Handle bins parameter manually (skipped by KwargsBuilder)
            if let Some(bins_value) = self.bins {
                match bins_value {
                    HistBins::Count(val) => kwargs.set_item("bins", val)?,
                    HistBins::Edges(arr) => kwargs.set_item("bins", arr.into_pyarray(py))?,
                    HistBins::Strategy(strategy) => kwargs.set_item("bins", strategy)?,
                }
            }

            // Handle range parameter manually (tuple)
            if let Some((lower, upper)) = self.range {
                kwargs.set_item("range", (lower, upper))?;
            }

            axes_obj.call_method("hist", (x_numpy,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    pub fn hist(&self, x: Array1<f64>) -> HistBuilder<'_> {
        HistBuilder {
            axes: self,
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
}
