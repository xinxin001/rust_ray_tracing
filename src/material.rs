use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Color},
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(a: Color) -> Self {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direciton = rec.normal + random_unit_vector();
        if scatter_direciton.near_zero() {
            scatter_direciton = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direciton);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Self {
        Metal { albedo: a, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(rec.p, reflected + random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;
        dot(scattered.direction(), rec.normal) > 0.0
    }
}
