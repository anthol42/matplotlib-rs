use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyPlotBuilder {
    x: Array1<f64>,
    y: Array1<f64>,
    // Format
    fmt: Option<String>,
    // Line styling
    color: Option<String>,
    linestyle: Option<String>,
    linewidth: Option<f64>,
    alpha: Option<f64>,
    label: Option<String>,
    antialiased: Option<bool>,
    // Marker styling
    marker: Option<String>,
    markersize: Option<f64>,
    markerfacecolor: Option<String>,
    markeredgecolor: Option<String>,
    markeredgewidth: Option<f64>,
    // Other
    zorder: Option<i32>,
    rasterized: Option<bool>,
}

impl PyPlotBuilder {
    pub fn new(x: Array1<f64>, y: Array1<f64>) -> Self {
        Self {
            x,
            y,
            fmt: None,
            color: None,
            linestyle: None,
            linewidth: None,
            alpha: None,
            label: None,
            antialiased: None,
            marker: None,
            markersize: None,
            markerfacecolor: None,
            markeredgecolor: None,
            markeredgewidth: None,
            zorder: None,
            rasterized: None,
        }
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            // Remove fmt from kwargs since it must be a positional argument
            let _ = kwargs.del_item("fmt");

            // Convert Array1 to NumPy arrays
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            // If fmt is provided, pass it as a positional argument
            if let Some(ref fmt_str) = self.fmt {
                ax.call_method("plot", (x_numpy, y_numpy, fmt_str), Some(&kwargs))?;
            } else {
                ax.call_method("plot", (x_numpy, y_numpy), Some(&kwargs))?;
            }

            Ok(())
        })
    }
}

/// Plot y versus x as lines and/or markers.
///
/// # Parameters
/// - `x`: Array1<f64>
/// - `y`: Array1<f64>
/// - `fmt`: Option<String>
/// - `color`: Option<String>
/// - `linestyle`: Option<String>
/// - `linewidth`: Option<f64>
/// - `alpha`: Option<f64>
/// - `label`: Option<String>
/// - `antialiased`: Option<bool>
/// - `marker`: Option<String>
/// - `markersize`: Option<f64>
/// - `markerfacecolor`: Option<String>
/// - `markeredgecolor`: Option<String>
/// - `markeredgewidth`: Option<f64>
/// - `zorder`: Option<i32>
/// - `rasterized`: Option<bool>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.plot.html>
pub fn plot(x: Array1<f64>, y: Array1<f64>) -> PyPlotBuilder {
    PyPlotBuilder::new(x, y)
}
