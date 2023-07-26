use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{dot, Point3},
};
pub struct Sphere {
    center: Point3,
    radius: f64,
    mat_ptr: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 0.0,
            mat_ptr: None,
        }
    }

    pub fn with_values(center: Point3, radius: f64, m: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat_ptr: Some(m),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true
    }
}
