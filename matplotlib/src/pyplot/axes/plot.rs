use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct PlotBuilder<'a> {
    axes: &'a Axes,
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

impl<'a> PlotBuilder<'a> {
    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Remove fmt from kwargs since it must be a positional argument
            // Ignore error if key doesn't exist
            let _ = kwargs.del_item("fmt");

            // Convert Array1 to NumPy arrays (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);
            let y_numpy = self.y.into_pyarray(py);

            // If fmt is provided, pass it as a positional argument
            if let Some(ref fmt_str) = self.fmt {
                axes_obj.call_method("plot", (x_numpy, y_numpy, fmt_str), Some(&kwargs))?;
            } else {
                axes_obj.call_method("plot", (x_numpy, y_numpy), Some(&kwargs))?;
            }

            Ok(())
        })
    }
}

impl Axes {
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
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.plot.html>
    pub fn plot(&self, x: Array1<f64>, y: Array1<f64>) -> PlotBuilder<'_> {
        PlotBuilder {
            axes: self,
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
}
