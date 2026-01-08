use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::ArrayD;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::shared::get_plt;

#[derive(KwargsBuilder)]
pub struct PyImshowBuilder {
    x: ArrayD<f64>,
    cmap: Option<String>,
    vmin: Option<f64>,
    vmax: Option<f64>,
    aspect: Option<String>,
    interpolation: Option<String>,
    origin: Option<String>,
    #[kwargs_builder(skip)]
    extent: Option<(f64, f64, f64, f64)>,
    alpha: Option<f64>,
    label: Option<String>,
}

impl PyImshowBuilder {
    pub fn new(x: ArrayD<f64>) -> Self {
        Self {
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

    pub fn extent(mut self, value: (f64, f64, f64, f64)) -> Self {
        self.extent = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            let x_numpy = self.x.into_pyarray(py);

            if let Some((left, right, bottom, top)) = self.extent {
                kwargs.set_item("extent", (left, right, bottom, top))?;
            }

            ax.call_method("imshow", (x_numpy,), Some(&kwargs))?;
            Ok(())
        })
    }
}

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
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.imshow.html>
pub fn imshow(x: ArrayD<f64>) -> PyImshowBuilder {
    PyImshowBuilder::new(x)
}
