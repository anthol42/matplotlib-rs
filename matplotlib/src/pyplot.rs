use pyo3::prelude::*;
pub mod subplots;
pub mod axes;
pub mod figure;
pub mod pyplot_api;

use pyo3::{PyResult, Python};
// Re-exports
pub use subplots::subplots;

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
    plot, scatter, hist, imshow, fill_between, bar, axhline, axvline, text, axis,
    xlabel, ylabel, title, xlim, ylim, xticks, yticks, legend, grid,
    suptitle, savefig, tight_layout, subplots_adjust,
    // Builders
    PyPlotBuilder, PyScatterBuilder, PyHistBuilder, PyImshowBuilder, PyFillBetweenBuilder,
    PyBarBuilder, PyAxhlineBuilder, PyAxvlineBuilder, PyTextBuilder, PyAxisBuilder,
    PyXlabelBuilder, PyYlabelBuilder, PyTitleBuilder, PyXlimBuilder, PyYlimBuilder,
    PyXticksBuilder, PyYticksBuilder, PyLegendBuilder, PyGridBuilder,
    PySuptitleBuilder, PySavefigBuilder, PyTightLayoutBuilder, PySubplotsAdjustBuilder,
    // Enums
    ScatterSize, ScatterColor, HistBins, FillBetweenY2, BarWidth, BarBottom, ErrorKwValue,
    AxisOption, XLabelLoc, YLabelLoc, TitleLoc,
    LegendLoc, LegendAlignment, GridWhich, GridAxis,
    SuptitleHAlign, SuptitleVAlign,
};
use pyplot_api::get_plt;
