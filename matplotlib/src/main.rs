use matplotlib::pyplot as plt;
use matplotlib::pyplot::axes::{AxisOption, GridWhich, GridAxis, XLabelLoc, YLabelLoc, TitleLoc, LegendLoc};
use ndarray::{arr1, Array2, Array3};
use std::collections::HashMap;
use matplotlib::pyplot::axes::AxisOption::Off;

fn main() {
    // Test here in the main thread - using 5x3 grid for more examples
    let (_, axes) = plt::subplots(5, 3).set().unwrap();

    // Plot example in top-left with axvline and axhline
    axes[(0,0)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .color("blue".to_string())
        .marker("o".to_string())
        .label("Quadratic".to_string())
        .set()
        .unwrap();

    // Add vertical line at x=2.0
    axes[(0,0)]
        .axvline(2.0)
        .color("red".to_string())
        .linestyle("--".to_string())
        .linewidth(2.0)
        .label("x=2".to_string())
        .set()
        .unwrap();

    // Add horizontal line at y=4.0
    axes[(0,0)]
        .axhline(4.0)
        .color("green".to_string())
        .linestyle("-.".to_string())
        .linewidth(2.0)
        .label("y=4".to_string())
        .set()
        .unwrap();

    // Add basic grid to first plot
    axes[(0,0)]
        .grid()
        .set()
        .unwrap();

    // Set x and y limits for first plot
    axes[(0,0)]
        .set_xlim()
        .left(0.5)
        .right(3.5)
        .set()
        .unwrap();

    axes[(0,0)]
        .set_ylim()
        .bottom(0.0)
        .top(10.0)
        .set()
        .unwrap();

    // Add axis labels with simple styling
    axes[(0,0)]
        .set_xlabel("X Values".to_string())
        .fontsize(10.0)
        .set()
        .unwrap();

    axes[(0,0)]
        .set_ylabel("Y Values".to_string())
        .fontsize(10.0)
        .fontweight("bold".to_string())
        .set()
        .unwrap();

    // Add simple title
    axes[(0,0)]
        .set_title("Plot with Lines".to_string())
        .fontsize(12.0)
        .fontweight("bold".to_string())
        .set()
        .unwrap();

    // Add simple legend
    axes[(0,0)]
        .legend()
        .loc(LegendLoc::UpperLeft)
        .fontsize(8.0)
        .set()
        .unwrap();

    // Set custom x-ticks with labels
    axes[(0,0)]
        .set_xticks(arr1(&[1.0, 2.0, 3.0]))
        .labels(vec!["One".to_string(), "Two".to_string(), "Three".to_string()])
        .fontsize(9.0)
        .set()
        .unwrap();

    // Scatter example 1: Simple scatter with single color and size in top-right
    axes[(0, 1)]
        .scatter(
            arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]),
            arr1(&[2.0, 4.0, 1.0, 5.0, 3.0])
        )
        .c("red")           // Single color (automatic &str conversion)
        .s(100.0)           // Single size (automatic f64 conversion)
        .marker("^".to_string())
        .alpha(0.7)
        .label("Simple Scatter".to_string())
        .set()
        .unwrap();

    // Add text annotation to scatter plot
    axes[(0, 1)]
        .text(3.0, 4.5, "Peak Value".to_string())
        .fontsize(10.0)
        .color("darkred".to_string())
        .fontweight("bold".to_string())
        .horizontalalignment("center".to_string())
        .set()
        .unwrap();

    // Plot example in bottom-left
    axes[(1, 0)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .fmt(".-.b".into())
        .set()
        .expect("Error here");

    // Scatter example 2: Variable sizes and color mapping with threshold lines
    let x = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let y = arr1(&[3.0, 1.0, 4.0, 2.0, 5.0, 3.5]);
    let sizes = arr1(&[50.0, 100.0, 150.0, 200.0, 250.0, 300.0]);
    let colors = arr1(&[0.1, 0.3, 0.5, 0.7, 0.9, 0.6]);  // Values for colormap

    axes[(0, 2)]
        .scatter(x, y)
        .s(sizes)           // Variable sizes (automatic Array1<f64> conversion)
        .c(colors)          // Color mapping (automatic Array1<f64> conversion)
        .cmap("viridis".to_string())
        .marker("o".to_string())
        .edgecolors("black".to_string())
        .linewidths(1.5)
        .label("Variable Size & Color".to_string())
        .set()
        .unwrap();

    // Add partial vertical line (middle 50% of height) at x=3.5
    axes[(0, 2)]
        .axvline(3.5)
        .ymin(0.25)
        .ymax(0.75)
        .color("orange".to_string())
        .linestyle(":".to_string())
        .linewidth(3.0)
        .alpha(0.7)
        .set()
        .unwrap();

    // Add partial horizontal line (middle 60% of width) at y=3.0
    axes[(0, 2)]
        .axhline(3.0)
        .xmin(0.2)
        .xmax(0.8)
        .color("purple".to_string())
        .linestyle(":".to_string())
        .linewidth(3.0)
        .alpha(0.7)
        .set()
        .unwrap();

    // Use axis method with Equal option for equal aspect ratio
    axes[(0, 2)]
        .axis(AxisOption::Equal)
        .set()
        .unwrap();

    // Fill between example 1: Basic fill between two curves
    let x_fill = arr1(&[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
    let y1 = arr1(&[1.0, 2.5, 2.0, 3.5, 3.0, 4.0]);
    let y2 = arr1(&[0.5, 1.0, 1.5, 2.0, 2.5, 3.0]);

    axes[(1, 1)]
        .fill_between(x_fill.clone(), y1.clone())
        .y2(y2.clone())     // Array (automatic Array1<f64> conversion)
        .alpha(0.3)
        .color("blue".to_string())
        .label("Filled Region".to_string())
        .set()
        .unwrap();

    // Plot the boundary lines for context
    axes[(1, 1)]
        .plot(x_fill.clone(), y1.clone())
        .color("blue".to_string())
        .linewidth(2.0)
        .set()
        .unwrap();

    axes[(1, 1)]
        .plot(x_fill.clone(), y2.clone())
        .color("blue".to_string())
        .linewidth(2.0)
        .set()
        .unwrap();

    // Add customized grid with styling
    axes[(1, 1)]
        .grid()
        .color("gray".to_string())
        .linestyle("--".to_string())
        .alpha(0.5)
        .set()
        .unwrap();

    // Add axis labels with color and position
    axes[(1, 1)]
        .set_xlabel("Data Points".to_string())
        .color("blue".to_string())
        .loc(XLabelLoc::Center)
        .set()
        .unwrap();

    axes[(1, 1)]
        .set_ylabel("Range".to_string())
        .color("blue".to_string())
        .fontstyle("italic".to_string())
        .set()
        .unwrap();

    // Add styled title with color
    axes[(1, 1)]
        .set_title("Fill Between Example".to_string())
        .fontsize(11.0)
        .color("darkblue".to_string())
        .fontstyle("italic".to_string())
        .set()
        .unwrap();

    // Add styled legend with custom appearance
    axes[(1, 1)]
        .legend()
        .loc(LegendLoc::Best)
        .framealpha(0.9)
        .facecolor("lightyellow".to_string())
        .edgecolor("blue".to_string())
        .shadow(true)
        .set()
        .unwrap();

    // Fill between example 2: Confidence band (fill from scalar y2=0)
    let x_conf = arr1(&[0.0, 0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0]);
    let y_mean = arr1(&[1.0, 1.2, 1.8, 2.0, 2.5, 2.8, 3.2, 3.5, 4.0]);
    let y_upper = arr1(&[1.5, 1.8, 2.5, 2.8, 3.2, 3.5, 4.0, 4.3, 5.0]);
    let y_lower = arr1(&[0.5, 0.6, 1.1, 1.2, 1.8, 2.1, 2.4, 2.7, 3.0]);

    axes[(1, 2)]
        .plot(x_conf.clone(), y_mean.clone())
        .color("red".to_string())
        .linewidth(2.0)
        .label("Mean".to_string())
        .set()
        .unwrap();

    axes[(1, 2)]
        .fill_between(x_conf.clone(), y_lower)
        .y2(y_upper)        // Array for upper bound
        .alpha(0.2)
        .color("red".to_string())
        .label("Confidence Band".to_string())
        .set()
        .unwrap();

    // Set only bottom limit (leaving top unchanged) for confidence plot
    axes[(1, 2)]
        .set_ylim()
        .bottom(0.0)
        .set()
        .unwrap();

    // Bar chart example 1: Simple bar chart with custom styling
    let categories = arr1(&[0.0, 1.0, 2.0, 3.0]);
    let values = arr1(&[23.0, 45.0, 56.0, 34.0]);

    axes[(2, 0)]
        .bar(categories, values)
        .width(0.6)         // Scalar width (automatic f64 conversion)
        .color("skyblue".to_string())
        .edgecolor("navy".to_string())
        .linewidth(1.5)
        .alpha(0.8)
        .label("Sales Data".to_string())
        .set()
        .unwrap();

    // Add rotated text annotation to bar chart
    axes[(2, 0)]
        .text(1.0, 50.0, "Highest Bar".to_string())
        .fontsize(9.0)
        .rotation(45.0)
        .color("navy".to_string())
        .fontstyle("italic".to_string())
        .set()
        .unwrap();

    // Add grid with which and axis options
    axes[(2, 0)]
        .grid()
        .which(GridWhich::Major)
        .axis(GridAxis::Y)
        .color("lightgray".to_string())
        .linestyle(":".to_string())
        .set()
        .unwrap();

    // Add styled axis labels with background
    axes[(2, 0)]
        .set_xlabel("Categories".to_string())
        .fontsize(11.0)
        .fontweight("bold".to_string())
        .backgroundcolor("lightyellow".to_string())
        .set()
        .unwrap();

    axes[(2, 0)]
        .set_ylabel("Sales".to_string())
        .fontsize(11.0)
        .color("navy".to_string())
        .loc(YLabelLoc::Center)
        .set()
        .unwrap();

    // Add positioned title with custom padding
    axes[(2, 0)]
        .set_title("Sales by Category".to_string())
        .loc(TitleLoc::Left)
        .fontsize(11.0)
        .fontweight("bold".to_string())
        .color("navy".to_string())
        .pad(10.0)
        .set()
        .unwrap();

    // Set custom y-ticks with labels
    axes[(2, 0)]
        .set_yticks(arr1(&[0.0, 20.0, 40.0, 60.0]))
        .labels(vec!["Low".to_string(), "Med".to_string(), "High".to_string(), "Max".to_string()])
        .fontsize(9.0)
        .color("navy".to_string())
        .set()
        .unwrap();

    // Bar chart example 2: Stacked bars
    let x_bar = arr1(&[0.0, 1.0, 2.0, 3.0, 4.0]);
    let heights1 = arr1(&[5.0, 7.0, 3.0, 8.0, 6.0]);
    let heights2 = arr1(&[3.0, 4.0, 6.0, 2.0, 5.0]);

    axes[(2, 1)]
        .bar(x_bar.clone(), heights1.clone())
        .bottom(0.0)        // Scalar bottom
        .color("lightgreen".to_string())
        .label("Category A".to_string())
        .set()
        .unwrap();

    axes[(2, 1)]
        .bar(x_bar, heights2)
        .bottom(heights1)   // Array bottom for stacking (automatic conversion)
        .color("lightcoral".to_string())
        .label("Category B".to_string())
        .set()
        .unwrap();

    // Add multi-column legend with custom positioning
    axes[(2, 1)]
        .legend()
        .ncols(2)
        .loc(LegendLoc::UpperCenter)
        .fontsize(9.0)
        .borderpad(0.5)
        .columnspacing(1.0)
        .set()
        .unwrap();

    // Set custom x-ticks for stacked bar chart
    axes[(2, 1)]
        .set_xticks(arr1(&[0.0, 1.0, 2.0, 3.0, 4.0]))
        .labels(vec!["Q1".to_string(), "Q2".to_string(), "Q3".to_string(), "Q4".to_string(), "Q5".to_string()])
        .fontsize(9.0)
        .rotation(45.0)
        .set()
        .unwrap();

    // Bar chart example 3: Bars with error bars and error_kw
    let x_err = arr1(&[1.0, 2.0, 3.0, 4.0]);
    let heights_err = arr1(&[20.0, 35.0, 30.0, 35.0]);
    let y_errors = arr1(&[2.0, 3.0, 2.5, 3.5]);
    let x_errors = arr1(&[0.1, 0.15, 0.1, 0.2]);

    // Create error_kw HashMap with custom error bar styling
    let mut error_styling = HashMap::new();
    error_styling.insert("elinewidth".to_string(), 2.0.into());
    error_styling.insert("capthick".to_string(), 2.0.into());

    axes[(2, 2)]
        .bar(x_err, heights_err)
        .yerr(y_errors)     // Y error bars
        .xerr(x_errors)     // X error bars
        .ecolor("red".to_string())
        .capsize(5.0)
        .error_kw(error_styling)
        .color("gold".to_string())
        .edgecolor("orange".to_string())
        .label("With Errors".to_string())
        .set()
        .unwrap();

    // Histogram example 1: Simple histogram with bin count
    let hist_data1 = arr1(&[
        1.0, 1.5, 2.0, 2.2, 2.5, 2.8, 3.0, 3.2, 3.5, 3.8,
        4.0, 4.1, 4.5, 4.8, 5.0, 5.2, 5.5, 5.8, 6.0, 6.5,
        2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 1.8, 2.3, 3.7, 4.2,
        5.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5, 6.0
    ]);

    axes[(3, 0)]
        .hist(hist_data1)
        .bins(15)           // HistBins::Count (automatic i32 conversion)
        .color("skyblue".to_string())
        .edgecolor("navy".to_string())
        .alpha(0.7)
        .label("Distribution".to_string())
        .set()
        .unwrap();

    // Add text with background color to histogram
    axes[(3, 0)]
        .text(4.5, 6.0, "Mean Area".to_string())
        .fontsize(11.0)
        .backgroundcolor("yellow".to_string())
        .alpha(0.8)
        .verticalalignment("center".to_string())
        .fontfamily("monospace".to_string())
        .set()
        .unwrap();

    // Add legend with bbox_to_anchor and title
    axes[(3, 0)]
        .legend()
        .bbox_to_anchor((1.05, 1.0))
        .title("Data".to_string())
        .title_fontsize(10.0)
        .fontsize(8.0)
        .frameon(true)
        .set()
        .unwrap();

    // Histogram example 2: Histogram with explicit bin edges
    let hist_data2 = arr1(&[
        0.5, 1.2, 1.8, 2.1, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0,
        5.5, 6.0, 6.5, 7.0, 8.0, 9.0, 10.0, 2.0, 3.0, 4.0,
        1.0, 2.0, 3.0, 5.0, 7.0, 8.0, 1.5, 2.5, 4.5, 6.5
    ]);
    let bin_edges = arr1(&[0.0, 2.0, 4.0, 6.0, 8.0, 10.0]);

    axes[(3, 1)]
        .hist(hist_data2)
        .bins(bin_edges)    // HistBins::Edges (automatic Array1<f64> conversion)
        .color("lightcoral".to_string())
        .edgecolor("darkred".to_string())
        .linewidth(1.5)
        .label("Custom Bins".to_string())
        .set()
        .unwrap();

    // Histogram example 3: Cumulative histogram with auto binning strategy
    let hist_data3 = arr1(&[
        1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 5.5,
        2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0, 1.5, 2.5, 3.5,
        4.5, 5.5, 3.0, 3.5, 4.0, 2.0, 3.0, 4.0, 5.0, 6.0
    ]);

    axes[(3, 2)]
        .hist(hist_data3)
        .bins("auto")       // HistBins::Strategy (automatic &str conversion)
        .cumulative(true)
        .density(true)
        .histtype("step".to_string())
        .color("green".to_string())
        .linewidth(2.0)
        .label("Cumulative".to_string())
        .set()
        .unwrap();

    // Image display example 1: Grayscale image with colormap
    let grayscale = Array2::from_shape_fn((50, 50), |(i, j)| {
        ((i * i + j * j) as f64).sqrt() / 70.0
    });

    axes[(4, 0)]
        .imshow(grayscale.into_dyn())
        .cmap("viridis".to_string())
        .interpolation("bilinear".to_string())
        .set()
        .unwrap();

    axes[(4, 0)]
        .text(25.0, -5.0, "Grayscale (viridis)".to_string())
        .fontsize(9.0)
        .horizontalalignment("center".to_string())
        .set()
        .unwrap();

    // Image display example 2: RGB image
    let rgb_image = Array3::from_shape_fn((40, 60, 3), |(i, j, c)| {
        match c {
            0 => i as f64 / 40.0,              // Red gradient
            1 => j as f64 / 60.0,              // Green gradient
            2 => 0.5,                          // Constant blue
            _ => 0.0,
        }
    });

    axes[(4, 1)]
        .imshow(rgb_image.into_dyn())
        .interpolation("nearest".to_string())
        .set()
        .unwrap();

    axes[(4, 1)]
        .text(30.0, -5.0, "RGB Image".to_string())
        .fontsize(9.0)
        .horizontalalignment("center".to_string())
        .set()
        .unwrap();

    // Image display example 3: Grayscale with extent and different colormap
    let pattern = Array2::from_shape_fn((30, 30), |(i, j)| {
        (((i as f64 / 5.0).sin() * (j as f64 / 5.0).cos()) + 1.0) / 2.0
    });

    axes[(4, 2)]
        .imshow(pattern.into_dyn())
        .cmap("hot".to_string())
        .extent((-3.0, 3.0, -3.0, 3.0))
        .origin("lower".to_string())
        .aspect("equal".to_string())
        .vmin(0.0)
        .vmax(1.0)
        .set()
        .unwrap();

    axes[(4, 2)]
        .text(0.0, -4.0, "Pattern (hot colormap)".to_string())
        .fontsize(9.0)
        .horizontalalignment("center".to_string())
        .set()
        .unwrap();

    axes[(4, 2)].axis(Off).set().unwrap();

    matplotlib::show().unwrap();
}