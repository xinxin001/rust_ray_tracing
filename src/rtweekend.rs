use std::f64;

use rand::{distributions::Uniform, prelude::Distribution, Rng};

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    let between = Uniform::from(0.0..1.0);
    let mut rng = rand::thread_rng();
    between.sample(&mut rng)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
