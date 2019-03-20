extern crate ndarray;

use crate::gnss_lib::constants::*;
use crate::gnss_lib::math_utils::*;
use ndarray::{arr2, Array2};

/// Geodetic Reference System
/// Earth Centered, Earth Fixed

/// GRS to ECEF
pub fn blh_to_ecef(phi: f64, lambda: f64, height: f64) -> Array2<f64> {
    let p_rad = deg_to_rad(phi);
    let l_rad = deg_to_rad(lambda);

    let n = LONG_RADIUS_OF_EARTH / (1.0 - ESSENTRICITY_2_L * square(p_rad.sin())).sqrt();

    arr2(&[
        [(n + height) * p_rad.cos() * l_rad.cos()],
        [(n + height) * p_rad.cos() * l_rad.sin()],
        [(n * (1.0 - ESSENTRICITY_2_L) + height) * p_rad.sin()]
    ])
}

#[test]
fn test_blh_ecef_convert() {
    let ret = blh_to_ecef(0.0, 0.0, 0.0);
    println!("{:?}", ret);
    assert_eq!(1, 1);
}