use ndarray::{Array, Array2};

fn main() {
    // Create a 2x3 array of zeros
    let mut a = Array::zeros((2, 3));

    // Set some values in the array
    a[(0, 1)] = 1.0;
    a[(1, 2)] = 2.0;

    // Print the array
    println!("{:?}", a);

    // Multiply the array by a scalar
    let b = &a * 2.0;

    // Print the result
    println!("{:?}", b);

    // Perform matrix multiplication with another array
    let c = Array2::from_shape_vec((3, 2), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]).unwrap();
    let d = a.dot(&c);
    
    // Print the result
    println!("{:?}", d);
}
