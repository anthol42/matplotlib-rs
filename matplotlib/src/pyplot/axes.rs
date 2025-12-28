mod plot;

use pyo3::{Py, PyAny};

pub struct Axes {
    pub inner: Py<PyAny>,
}
