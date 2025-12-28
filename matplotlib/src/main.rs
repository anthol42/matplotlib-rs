use matplotlib::pyplot as plt;
use ndarray::arr1;

fn main() {
    // Test here in the main thread
    let (_, axes) = plt::subplots(2, 2).set().unwrap();

    axes[(0,0)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .color(Some("blue".to_string()))
        .marker(Some("o".to_string()))
        .label(Some("Quadratic".to_string()))
        .set()
        .unwrap();
    axes[(1, 0)].plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[1.0, 4.0, 9.0]))
        .fmt(Some(".-.b".into()))
        .set().expect("Error here");

    axes[(1,1)]
        .plot(arr1(&[1.0, 2.0, 3.0]), arr1(&[9.0, 4.0, 1.0]))
        .color(Some("red".to_string()))
        .linestyle(Some("--".to_string()))
        .linewidth(Some(2.0))
        .set()
        .unwrap();

    matplotlib::show().unwrap();
}