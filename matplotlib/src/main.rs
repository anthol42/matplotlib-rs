use matplotlib::pyplot as plt;

fn main() {
    // Test here in the main thread
    let (_, axes) = plt::subplots(2, 2).set().unwrap();
    axes[(0,0)].plot(vec![1.0, 2.0, 3.0], vec![1.0, 4.0, 9.0]).unwrap();
    axes[(1,1)].plot(vec![1.0, 2.0, 3.0], vec![9.0, 4.0, 1.0]).unwrap();
    matplotlib::show().unwrap();
}