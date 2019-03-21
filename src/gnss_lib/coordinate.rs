extern crate ndarray;
extern crate ndarray_linalg;

use crate::gnss_lib::constants::*;
use crate::gnss_lib::math_utils::*;
use ndarray::*;
use ndarray_linalg::*;
use std::f64;

/// Geodetic Reference System
/// Earth Centered, Earth Fixed

/// GRS to ECEF
pub fn blh_to_ecef(blh: &Array2<f64>) -> Array2<f64> {
    let (phi, lambda, height) = (blh[[0, 0]], blh[[1, 0]], blh[[2, 0]]);
    let p_rad = deg_to_rad(phi);
    let l_rad = deg_to_rad(lambda);

    let n = LONG_RADIUS_OF_EARTH / (1.0 - ESSENTRICITY_2_L * square(p_rad.sin())).sqrt();

    arr2(&[
        [(n + height) * p_rad.cos() * l_rad.cos()],
        [(n + height) * p_rad.cos() * l_rad.sin()],
        [(n * (1.0 - ESSENTRICITY_2_L) + height) * p_rad.sin()]
    ])
}

pub fn ecef_to_blh(ecef: &Array2<f64>) -> Array2<f64> {
    let (x, y, z) = (ecef[[0, 0]], ecef[[1, 0]], ecef[[2, 0]]);
    let p = (x*x + y*y).sqrt();
    let sita = (z * LONG_RADIUS_OF_EARTH).atan2(p * SHORT_RADIUS_OF_EARTH);

    let phi = rad_to_deg((z + ESSENTRICITY_2_S * SHORT_RADIUS_OF_EARTH * cube(sita.sin())).atan2(p - ESSENTRICITY_2_L * LONG_RADIUS_OF_EARTH * cube(sita.cos())));
    let lambda = rad_to_deg(y.atan2(x));

    let p_rad = deg_to_rad(phi);
    let n = LONG_RADIUS_OF_EARTH / (1.0 - ESSENTRICITY_2_L * square(p_rad.sin())).sqrt();
    let height = p / p_rad.cos() - n;

    arr2(&[[phi], [lambda], [height]])
}

#[test]
fn test_blh_ecef_convert() {
    let (lat, lon, hig) = (38.13579617, 140.91581617, 41.940);
    let v = arr2(&[[lat], [lon], [hig]]);
    let ret = ecef_to_blh(&blh_to_ecef(&v));
    assert_near_vec!(&v, &ret, f64::EPSILON * 1e10);
}