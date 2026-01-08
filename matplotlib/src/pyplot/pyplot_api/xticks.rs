use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyList;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyXticksBuilder {
    ticks: Array1<f64>,
    #[kwargs_builder(skip)]
    labels: Option<Vec<String>>,
    minor: Option<bool>,
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    color: Option<String>,
    rotation: Option<f64>,
}

impl PyXticksBuilder {
    pub fn new(ticks: Array1<f64>) -> Self {
        Self {
            ticks,
            labels: None,
            minor: None,
            fontsize: None,
            fontweight: None,
            fontstyle: None,
            fontfamily: None,
            color: None,
            rotation: None,
        }
    }

    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            let ticks_list = PyList::new(py, self.ticks.iter())?;

            if let Some(labels_vec) = self.labels {
                let labels_list = PyList::new(py, labels_vec.iter())?;
                ax.call_method("set_xticks", (ticks_list, labels_list), Some(&kwargs))?;
            } else {
                ax.call_method("set_xticks", (ticks_list,), Some(&kwargs))?;
            }

            Ok(())
        })
    }
}

/// Get or set the current tick locations and labels of the x-axis.
///
/// # Parameters
/// - `ticks`: Array1<f64>
/// - `labels`: Option<Vec<String>>
/// - `minor`: Option<bool>
/// - `fontsize`: Option<f64>
/// - `fontweight`: Option<String>
/// - `fontstyle`: Option<String>
/// - `fontfamily`: Option<String>
/// - `color`: Option<String>
/// - `rotation`: Option<f64>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.xticks.html>
pub fn xticks(ticks: Array1<f64>) -> PyXticksBuilder {
    PyXticksBuilder::new(ticks)
}
