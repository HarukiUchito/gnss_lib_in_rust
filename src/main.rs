extern crate ndarray;
extern crate ndarray_linalg;
use ndarray::*;
use ndarray_linalg::*;
use gnss_practice_in_rust::gnss_lib::coordinate::*;
use std::f64;

fn main() {
    let (lat, lon, hig) = (38.13579617, 140.91581617, 41.940);
    let v = arr2(&[[lat], [lon], [hig]]);
    let ret = ecef_to_blh(&blh_to_ecef(&v));
    println!("{:?}", (v - ret).norm_l2());
    println!("{:?}", f64::EPSILON * 1e10);
}
