use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;

use super::Axes;

impl Axes {
    pub fn plot(&self, x: Vec<f64>, y: Vec<f64>) -> PyResult<()> {
        Python::attach(|py| {
            let axes = self.inner.bind(py);
            axes.call_method("plot", (x, y), None)?;
            Ok(())
        })
    }
}