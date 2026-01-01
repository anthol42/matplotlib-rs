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
    /// # Arguments
    /// * `ticks` - Array of tick locations (required)
    ///
    /// Optional parameters via builder pattern:
    /// * `labels` - Custom tick labels (must match ticks length)
    /// * `minor` - If true, set minor ticks instead of major ticks (default: false)
    ///
    /// Text properties (only when labels are provided):
    /// * `fontsize` - Font size in points
    /// * `fontweight` - Font weight (e.g., "bold", "normal")
    /// * `fontstyle` - Font style (e.g., "italic", "normal")
    /// * `fontfamily` - Font family (e.g., "serif", "monospace")
    /// * `color` - Text color
    /// * `rotation` - Rotation angle in degrees
    ///
    /// # Examples
    /// ```ignore
    /// // Set tick locations only
    /// axes.set_yticks(arr1(&[0.0, 2.0, 4.0, 6.0, 8.0, 10.0]))
    ///     .set()?;
    ///
    /// // Set ticks with custom labels
    /// axes.set_yticks(arr1(&[0.0, 25.0, 50.0, 75.0, 100.0]))
    ///     .labels(vec!["0%".to_string(), "25%".to_string(), "50%".to_string(),
    ///                  "75%".to_string(), "100%".to_string()])
    ///     .fontsize(10.0)
    ///     .color("red".to_string())
    ///     .set()?;
    ///
    /// // Set minor ticks
    /// axes.set_yticks(arr1(&[1.0, 3.0, 5.0, 7.0, 9.0]))
    ///     .minor(true)
    ///     .set()?;
    /// ```
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
