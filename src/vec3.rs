use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

use crate::rtweekend::{random_double, random_double_range};

pub type Point3 = Vec3; // 3D point
pub type Color = Vec3; // RGB color

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f64 {
        self[0]
    }
    pub fn y(&self) -> f64 {
        self[1]
    }
    pub fn z(&self) -> f64 {
        self[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.e.iter().map(|e| e * e).sum()
    }

    pub fn random() -> Self {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        return Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        );
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        e: [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ],
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    return v / v.length();
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        return in_unit_sphere;
    }
    -in_unit_sphere
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            e: [-self[0], -self[1], -self[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [self[0] + other[0], self[1] + other[1], self[2] + other[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [self[0] - other[0], self[1] - other[1], self[2] - other[2]],
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [self[0] * other[0], self[1] * other[1], self[2] * other[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self {
            e: [self[0] * t, self[1] * t, self[2] * t],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        Self {
            e: [self[0] / t, self[1] / t, self[2] / t],
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}
