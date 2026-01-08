use pyo3::prelude::*;
use kwargs_builder_derive::KwargsBuilder;
use ndarray::{Array2};

use crate::pyplot::axes::Axes;
use crate::pyplot::figure::Figure;
use crate::pyplot::pyplot_api::get_plt;

#[derive(KwargsBuilder)]
pub struct SubPlotsBuilder {
    nrows: usize,
    ncols: usize,
    sharex: Option<bool>,
    sharey: Option<bool>,
    width_ratios: Option<Vec<f64>>,
    height_ratios: Option<Vec<f64>>,
}

impl SubPlotsBuilder {
    pub fn set(self) -> PyResult<(Figure, Array2<Axes>)> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let kwargs = self.get_kwargs(py)?;
            kwargs.set_item("squeeze", false)?;

            let result = plt.call_method(
                "subplots",
                (self.nrows, self.ncols),
                Some(&kwargs)
            )?;

            let (fig, axes_obj): (Bound<PyAny>, Bound<PyAny>) = result.extract()?;

            let axes_2d: Vec<Vec<Bound<PyAny>>> = axes_obj.extract()?;
            let mut axes_vec = Vec::with_capacity(self.nrows * self.ncols);

            for row in axes_2d {
                for ax in row {
                    axes_vec.push(Axes { inner: ax.unbind() });
                }
            }

            let array = Array2::from_shape_vec((self.nrows, self.ncols), axes_vec)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Failed to create array: {}", e)
                ))?;

            Ok((
                Figure { inner: fig.unbind() },
                array
            ))
        })
    }

}

/// Create a figure and a set of subplots.
///
/// # Parameters
/// - `nrows`: usize
/// - `ncols`: usize
/// - `sharex`: Option<bool>
/// - `sharey`: Option<bool>
/// - `width_ratios`: Option<Vec<f64>>
/// - `height_ratios`: Option<Vec<f64>>
///
/// # Returns
/// Returns a tuple of (Figure, Array2<Axes>)
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.subplots.html>
pub fn subplots(nrows: usize, ncols: usize) -> SubPlotsBuilder {
    SubPlotsBuilder {
        nrows,
        ncols,
        sharex: None,
        sharey: None,
        width_ratios: None,
        height_ratios: None,
    }
}


