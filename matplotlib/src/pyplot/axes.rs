mod plot;
mod scatter;
mod fill_between;
mod bar;
mod axvline;
mod axhline;
mod hist;
mod text;
mod imshow;

use pyo3::{Py, PyAny};

pub struct Axes {
    pub inner: Py<PyAny>,
}
