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
mod set_xlabel;
mod set_ylabel;
mod set_title;
mod legend;

use pyo3::{Py, PyAny};

// Re-export AxisOption for user convenience
pub use axis::AxisOption;
// Re-export GridWhich and GridAxis for user convenience
pub use grid::{GridWhich, GridAxis};
// Re-export XLabelLoc and YLabelLoc for user convenience
pub use set_xlabel::XLabelLoc;
pub use set_ylabel::YLabelLoc;
// Re-export TitleLoc for user convenience
pub use set_title::TitleLoc;
// Re-export LegendLoc and LegendAlignment for user convenience
pub use legend::{LegendLoc, LegendAlignment};

pub struct Axes {
    pub inner: Py<PyAny>,
}
