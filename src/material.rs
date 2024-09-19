use crate::{
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{
        dot, random_in_unit_sphere, random_unit_vector, reflect, refract, unit_vector, Color, Vec3,
    },
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
        _: &Ray,
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

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Dielectric {
            refraction_index: ri,
        }
    }

    pub fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        // Schlick's approximation for reflectance
        // https://en.wikipedia.org/wiki/Schlick's_approximation
        let r0 = f64::sqrt((1.0 - refraction_index) / (1.0 + refraction_index));
        return r0 + (1.0 - r0) * f64::powf(1. + cosine, 5.);
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(r_in.direction());
        let cos_theta = f64::min(dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || self.reflectance(cos_theta, ri) > random_double() {
            // Must reflect
            direction = reflect(&unit_direction, &rec.normal)
        } else {
            // Must refract
            direction = refract(&unit_direction, &rec.normal, ri);
        }
        *scattered = Ray::new(rec.p, direction);
        true
    }
}
