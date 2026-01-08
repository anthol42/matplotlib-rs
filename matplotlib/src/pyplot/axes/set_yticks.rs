use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyList;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct SetYticksBuilder<'a> {
    axes: &'a Axes,
    ticks: Array1<f64>,
    #[kwargs_builder(skip)]
    labels: Option<Vec<String>>,
    minor: Option<bool>,
    // Text properties (only allowed when labels are provided)
    fontsize: Option<f64>,
    fontweight: Option<String>,
    fontstyle: Option<String>,
    fontfamily: Option<String>,
    color: Option<String>,
    rotation: Option<f64>,
}

impl<'a> SetYticksBuilder<'a> {
    /// Set custom tick labels.
    /// Must have the same length as ticks.
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert ticks array to Python list
            let ticks_list = PyList::new(py, self.ticks.iter())?;

            // Handle labels parameter if specified
            if let Some(labels_vec) = self.labels {
                let labels_list = PyList::new(py, labels_vec.iter())?;
                axes_obj.call_method("set_yticks", (ticks_list, labels_list), Some(&kwargs))?;
            } else {
                axes_obj.call_method("set_yticks", (ticks_list,), Some(&kwargs))?;
            };

            Ok(())
        })
    }
}

impl Axes {
    /// Set the y-axis tick locations and optionally labels.
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
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.set_yticks.html>
    pub fn set_yticks(&self, ticks: Array1<f64>) -> SetYticksBuilder<'_> {
        SetYticksBuilder {
            axes: self,
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
}
