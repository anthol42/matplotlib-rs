use pyo3::{Py, PyAny, PyResult, Python};
use std::sync::OnceLock;

/// Global pyplot module instance shared across all pyplot API functions
static PLT: OnceLock<Py<PyAny>> = OnceLock::new();

/// Get the shared pyplot module, initializing it if necessary
pub fn get_plt() -> PyResult<Py<PyAny>> {
    Python::attach(|py| {
        if let Some(plt) = PLT.get() {
            Ok(plt.clone_ref(py))
        } else {
            let plt_module = py.import("matplotlib.pyplot")?;
            let plt: Py<PyAny> = plt_module.unbind().into();
            PLT.set(plt.clone_ref(py)).ok();
            Ok(plt)
        }
    })
}
