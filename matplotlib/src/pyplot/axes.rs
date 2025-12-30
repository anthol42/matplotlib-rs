mod plot;
mod scatter;

use pyo3::{Py, PyAny};

pub struct Axes {
    pub inner: Py<PyAny>,
}
