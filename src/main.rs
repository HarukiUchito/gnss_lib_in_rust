extern crate ndarray;
use gnss_practice_in_rust::gnss_lib::coordinate::*;
use ndarray::{arr2};

fn main() {
    let ret = blh_to_ecef(0.0, 0.0, 0.0);
    println!("{:?}", ret);
    let matrix1 = arr2(&[[1.0, 2.0, 3.0],
                         [4.0, 5.0, 6.0],
                         [7.0, 8.0, 9.0]]);
    let matrix2 = arr2(&[[1.0, 0.0, 0.0],
                         [0.0, 1.0, 0.0],
                         [0.0, 0.0, 2.0]]);
    let mut result = matrix1.dot(&matrix2);
    println!("{:?}", result);
    result[[1, 1]] = 0.4;
    println!("{:?}", result);
}
