use matplotlib::pyplot as plt;
use matplotlib::pyplot::{LegendLoc, TitleLoc, XLabelLoc, GridWhich, GridAxis};
use ndarray::arr1;

fn main() {
    // Simple pyplot API example - stateful plotting like Python's matplotlib.pyplot

    // Create some data
    let x = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);
    let y1 = arr1(&[1.0, 4.0, 9.0, 16.0, 25.0]);
    let y2 = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0]);

    // Plot first line with full parameters
    plt::plot(x.clone(), y1)
        .color("blue".to_string())
        .marker("o".to_string())
        .label("Quadratic".to_string())
        .linewidth(2.0)
        .markersize(8.0)
        .alpha(0.8)
        .set()
        .unwrap();

    // Plot second line
    plt::plot(x.clone(), y2)
        .color("red".to_string())
        .marker("s".to_string())
        .label("Linear".to_string())
        .linestyle("--".to_string())
        .linewidth(2.0)
        .set()
        .unwrap();

    // Scatter plot with full parameters
    let x_scatter = arr1(&[1.5, 2.5, 3.5, 4.5]);
    let y_scatter = arr1(&[2.0, 6.0, 12.0, 20.0]);

    plt::scatter(x_scatter, y_scatter)
        .s(100.0)
        .c("green")
        .marker("^".to_string())
        .alpha(0.6)
        .edgecolors("darkgreen".to_string())
        .linewidths(1.5)
        .label("Points".to_string())
        .set()
        .unwrap();

    // Add labels with full parameters
    plt::xlabel("X Values".to_string())
        .fontsize(12.0)
        .fontweight("bold".to_string())
        .color("darkblue".to_string())
        .loc(XLabelLoc::Center)
        .set()
        .unwrap();

    plt::ylabel("Y Values".to_string())
        .fontsize(12.0)
        .fontweight("bold".to_string())
        .color("darkred".to_string())
        .fontstyle("italic".to_string())
        .set()
        .unwrap();

    // Add title with full parameters
    plt::title("Pyplot API Example".to_string())
        .fontsize(16.0)
        .fontweight("bold".to_string())
        .color("darkblue".to_string())
        .loc(TitleLoc::Center)
        .pad(15.0)
        .set()
        .unwrap();

    // Set axis limits
    plt::xlim()
        .left(0.0)
        .right(6.0)
        .set()
        .unwrap();

    plt::ylim()
        .bottom(0.0)
        .top(30.0)
        .set()
        .unwrap();

    // Add legend with full parameters
    plt::legend()
        .loc(LegendLoc::UpperLeft)
        .fontsize(10.0)
        .framealpha(0.9)
        .shadow(true)
        .facecolor("lightyellow".to_string())
        .edgecolor("black".to_string())
        .set()
        .unwrap();

    // Add grid with full parameters
    plt::grid()
        .visible(true)
        .which(GridWhich::Major)
        .axis(GridAxis::Both)
        .color("gray".to_string())
        .linestyle("--".to_string())
        .alpha(0.5)
        .set()
        .unwrap();

    // Show the plot
    plt::show().unwrap();
}
