use std::f64;
use std::ops::*;

pub const PI: f64 = f64::consts::PI;

#[inline]
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

#[inline]
pub fn square<T>(v: T) -> T
where T: Mul<Output = T> + Copy
{
    v * v
}