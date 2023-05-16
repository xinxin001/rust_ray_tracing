use crate::vec3::{Point3, Vec3};

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        return self.origin + self.direction * t;
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }
}
