use matplotlib::pyplot as plt;
use ndarray::arr1;

fn main() {
    // Test here in the main thread - using 2x3 grid for more examples
    let (_, axes) = plt::subplots(2, 3).set().unwrap();

    // Plot example in top-left
    axes[(0,0)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .color("blue".to_string())
        .marker("o".to_string())
        .label("Quadratic".to_string())
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

    // Plot example in bottom-left
    axes[(1, 0)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .fmt(".-.b".into())
        .set()
        .expect("Error here");

    // Scatter example 2: Variable sizes and color mapping
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

    matplotlib::show().unwrap();
}