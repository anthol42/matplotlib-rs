use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::ArrayD;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::Axes;

#[derive(KwargsBuilder)]
pub struct ImshowBuilder<'a> {
    axes: &'a Axes,
    x: ArrayD<f64>,
    // Colormap & Normalization (for grayscale)
    cmap: Option<String>,
    vmin: Option<f64>,
    vmax: Option<f64>,
    // Display Options
    aspect: Option<String>,
    interpolation: Option<String>,
    origin: Option<String>,
    // Coordinate System (extent handled manually as tuple)
    #[kwargs_builder(skip)]
    extent: Option<(f64, f64, f64, f64)>,
    // Styling
    alpha: Option<f64>,
    // Other
    label: Option<String>,
}

impl<'a> ImshowBuilder<'a> {
    // Manual method for extent (tuple)
    pub fn extent(mut self, value: (f64, f64, f64, f64)) -> Self {
        self.extent = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let axes_obj = self.axes.inner.bind(py);
            let kwargs = self.get_kwargs(py)?;

            // Convert ArrayD to NumPy array (move ownership, no clone needed)
            let x_numpy = self.x.into_pyarray(py);

            // Handle extent parameter manually (tuple)
            if let Some((left, right, bottom, top)) = self.extent {
                kwargs.set_item("extent", (left, right, bottom, top))?;
            }

            axes_obj.call_method("imshow", (x_numpy,), Some(&kwargs))?;

            Ok(())
        })
    }
}

impl Axes {
    /// Display data as an image, i.e., on a 2D regular raster.
    ///
    /// # Parameters
    /// - `x`: ArrayD<f64> - 2D array (grayscale) or 3D array (RGB/RGBA)
    /// - `cmap`: Option<String>
    /// - `vmin`: Option<f64>
    /// - `vmax`: Option<f64>
    /// - `aspect`: Option<String>
    /// - `interpolation`: Option<String>
    /// - `origin`: Option<String>
    /// - `extent`: Option<(f64, f64, f64, f64)> - (left, right, bottom, top)
    /// - `alpha`: Option<f64>
    /// - `label`: Option<String>
    ///
    /// # See Also
    /// <https://matplotlib.org/stable/api/_as_gen/matplotlib.axes.Axes.imshow.html>
    pub fn imshow(&self, x: ArrayD<f64>) -> ImshowBuilder<'_> {
        ImshowBuilder {
            axes: self,
            x,
            cmap: None,
            vmin: None,
            vmax: None,
            aspect: None,
            interpolation: None,
            origin: None,
            extent: None,
            alpha: None,
            label: None,
        }
    }
}
