mod plot;
mod scatter;
mod fill_between;
mod bar;
mod axvline;
mod axhline;
mod hist;
mod text;
mod imshow;
mod axis;
mod grid;
mod set_xlim;
mod set_ylim;

use pyo3::{Py, PyAny};

// Re-export AxisOption for user convenience
pub use axis::AxisOption;
// Re-export GridWhich and GridAxis for user convenience
pub use grid::{GridWhich, GridAxis};

pub struct Axes {
    pub inner: Py<PyAny>,
}
