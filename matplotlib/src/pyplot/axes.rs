pub mod plot;
pub mod scatter;
pub mod fill_between;
pub mod bar;
pub mod axvline;
pub mod axhline;
pub mod hist;
pub mod text;
pub mod imshow;
pub mod axis;
pub mod grid;
pub mod set_xlim;
pub mod set_ylim;
pub mod set_xlabel;
pub mod set_ylabel;
pub mod set_title;
pub mod legend;
pub mod set_xticks;
pub mod set_yticks;

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
