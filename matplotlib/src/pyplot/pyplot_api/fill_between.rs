use pyo3::{PyResult, Python};
use pyo3::prelude::PyAnyMethods;
use ndarray::Array1;
use kwargs_builder_derive::KwargsBuilder;
use numpy::IntoPyArray;

use super::shared::get_plt;

pub enum FillBetweenY2 {
    Scalar(f64),
    Array(Array1<f64>),
}

impl From<f64> for FillBetweenY2 {
    fn from(value: f64) -> Self {
        FillBetweenY2::Scalar(value)
    }
}

impl From<Array1<f64>> for FillBetweenY2 {
    fn from(value: Array1<f64>) -> Self {
        FillBetweenY2::Array(value)
    }
}

#[derive(KwargsBuilder)]
pub struct PyFillBetweenBuilder {
    x: Array1<f64>,
    y1: Array1<f64>,
    #[kwargs_builder(skip)]
    y2: Option<FillBetweenY2>,
    #[kwargs_builder(skip)]
    where_mask: Option<Array1<bool>>,
    interpolate: Option<bool>,
    alpha: Option<f64>,
    color: Option<String>,
    edgecolor: Option<String>,
    linewidth: Option<f64>,
    step: Option<String>,
    label: Option<String>,
    zorder: Option<i32>,
    hatch: Option<String>,
}

impl PyFillBetweenBuilder {
    pub fn new(x: Array1<f64>, y1: Array1<f64>) -> Self {
        Self {
            x,
            y1,
            y2: None,
            where_mask: None,
            interpolate: None,
            alpha: None,
            color: None,
            edgecolor: None,
            linewidth: None,
            step: None,
            label: None,
            zorder: None,
            hatch: None,
        }
    }

    pub fn y2<T: Into<FillBetweenY2>>(mut self, value: T) -> Self {
        self.y2 = Some(value.into());
        self
    }

    pub fn where_mask(mut self, value: Array1<bool>) -> Self {
        self.where_mask = Some(value);
        self
    }

    pub fn set(self) -> PyResult<()> {
        Python::attach(|py| {
            let plt_obj = get_plt()?;
            let plt = plt_obj.bind(py);
            let ax = plt.call_method0("gca")?;
            let kwargs = self.get_kwargs(py)?;

            let x_numpy = self.x.into_pyarray(py);
            let y1_numpy = self.y1.into_pyarray(py);

            if let Some(y2_value) = self.y2 {
                match y2_value {
                    FillBetweenY2::Scalar(val) => kwargs.set_item("y2", val)?,
                    FillBetweenY2::Array(arr) => kwargs.set_item("y2", arr.into_pyarray(py))?,
                }
            }

            if let Some(mask) = self.where_mask {
                kwargs.set_item("where", mask.into_pyarray(py))?;
            }

            ax.call_method("fill_between", (x_numpy, y1_numpy), Some(&kwargs))?;
            Ok(())
        })
    }
}

/// Fill the area between two horizontal curves.
///
/// # Parameters
/// - `x`: Array1<f64>
/// - `y1`: Array1<f64>
/// - `y2`: Option<FillBetweenY2> - Second y values (scalar or array, defaults to 0)
/// - `where_mask`: Option<Array1<bool>> - Boolean array to define where to fill
/// - `interpolate`: Option<bool>
/// - `alpha`: Option<f64>
/// - `color`: Option<String>
/// - `edgecolor`: Option<String>
/// - `linewidth`: Option<f64>
/// - `step`: Option<String>
/// - `label`: Option<String>
/// - `zorder`: Option<i32>
/// - `hatch`: Option<String>
///
/// # See Also
/// <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.fill_between.html>
pub fn fill_between(x: Array1<f64>, y1: Array1<f64>) -> PyFillBetweenBuilder {
    PyFillBetweenBuilder::new(x, y1)
}
