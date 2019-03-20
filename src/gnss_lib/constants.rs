/// The speed of light [m / s]
pub const SPEED_OF_LIGHT: f64 = 2.99792458e+8;

/// The flattening of the earth [s / m^(1/2)]
pub const FLATTENING: f64 = -4.442807633e-10;

/// The long radius of the earth on the equator \[m\]
pub const LONG_RADIUS_OF_EARTH: f64 = 6378137.0;

/// The short radius of the earth on the equator \[m\]
pub const SHORT_RADIUS_OF_EARTH: f64 = LONG_RADIUS_OF_EARTH * (1.0 - FLATTENING);

/// The rotation rate of the earth [rad / s]
pub const ROTATION_RATE: f64 = 7.2921151467e-05;

/// The universal gravity of the earth [m^3 / s^2]
pub const UNIVERSAL_GRAVITY: f64 = 3.986005e+14;

