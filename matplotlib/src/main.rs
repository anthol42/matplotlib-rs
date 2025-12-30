use matplotlib::pyplot as plt;
use ndarray::arr1;

fn main() {
    // Test here in the main thread
    let (_, axes) = plt::subplots(2, 2).set().unwrap();

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

    // Scatter example 2: Variable sizes and color mapping in bottom-right
    let x = arr1(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    let y = arr1(&[3.0, 1.0, 4.0, 2.0, 5.0, 3.5]);
    let sizes = arr1(&[50.0, 100.0, 150.0, 200.0, 250.0, 300.0]);
    let colors = arr1(&[0.1, 0.3, 0.5, 0.7, 0.9, 0.6]);  // Values for colormap

    axes[(1,1)]
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

    matplotlib::show().unwrap();
}