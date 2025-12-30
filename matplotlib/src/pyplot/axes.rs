mod plot;
mod scatter;
mod fill_between;
mod bar;

use pyo3::{Py, PyAny};

pub struct Axes {
    pub inner: Py<PyAny>,
}
