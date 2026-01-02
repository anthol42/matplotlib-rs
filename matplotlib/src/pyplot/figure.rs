pub mod suptitle;
pub mod savefig;
pub mod tight_layout;
pub mod subplots_adjust;

use pyo3::{Py, PyAny};

// Re-export alignment enums for user convenience
pub use suptitle::{SuptitleHAlign, SuptitleVAlign};

pub struct Figure {
    pub inner: Py<PyAny>,
}
