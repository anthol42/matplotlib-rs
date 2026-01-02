mod suptitle;
mod savefig;
mod tight_layout;
mod subplots_adjust;

use pyo3::{Py, PyAny};

// Re-export alignment enums for user convenience
pub use suptitle::{SuptitleHAlign, SuptitleVAlign};

pub struct Figure {
    pub inner: Py<PyAny>,
}
