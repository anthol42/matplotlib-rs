use pyo3::prelude::*;
pub mod pyplot;

pub fn show() -> PyResult<()> {
    Python::attach(|py| {
        let plt = py.import("matplotlib.pyplot")?;
        plt.call_method0("show")?;

        Ok(())
    })
}

