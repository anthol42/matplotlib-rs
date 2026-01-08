use pyo3::prelude::*;
pub mod axes;
pub mod figure;
pub mod pyplot_api;

use pyo3::{PyResult, Python};
// Re-exports
pub use pyplot_api::subplots::subplots;

pub fn show() -> PyResult<()> {
    Python::attach(|py| {
        let plt_obj = get_plt()?;
        let plt = plt_obj.bind(py);
        plt.call_method0("show")?;
        Ok(())
    })
}

// Re-export pyplot API
pub use pyplot_api::{
    // Functions
    axhline, axis, axvline, bar, fill_between, grid, hist, imshow, legend, plot,
    savefig, scatter, subplots_adjust, suptitle, text, tight_layout, title, xlabel, xlim,
    xticks, ylabel, ylim, yticks,
    // Builders
    AxisOption, BarBottom, BarWidth, ErrorKwValue, FillBetweenY2,
    GridAxis, GridWhich, HistBins, LegendAlignment, LegendLoc,
    PyAxhlineBuilder, PyAxisBuilder, PyAxvlineBuilder, PyBarBuilder, PyFillBetweenBuilder,
    PyGridBuilder, PyHistBuilder, PyImshowBuilder, PyLegendBuilder,
    PyPlotBuilder, PySavefigBuilder, PyScatterBuilder, PySubplotsAdjustBuilder,
    // Enums
    PySuptitleBuilder, PyTextBuilder, PyTightLayoutBuilder, PyTitleBuilder, PyXlabelBuilder, PyXlimBuilder, PyXticksBuilder,
    PyYlabelBuilder, PyYlimBuilder, PyYticksBuilder, ScatterColor,
    ScatterSize, SuptitleHAlign, SuptitleVAlign, TitleLoc,
    XLabelLoc, YLabelLoc,
};
use pyplot_api::get_plt;
