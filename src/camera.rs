use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    rtweekend::random_double,
    rtweekend::INFINITY,
    vec3::{unit_vector, Color, Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    aspect_ratio: f64,
    pub image_width: usize,
    pub image_height: usize,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn new() -> Self {
        // Image
        let aspect_ratio = 1.;
        let image_width: usize = 400;
        let mut image_height: usize = (image_width as f64 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };
        let samples_per_pixel = 10;
        let max_depth = 10;

        let aspect_ratio = 16.0 / 9.0;
        let focal_length = 1.0;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        );
    }

    fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
        let mut rec = HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(r, 0.001, INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if rec
                .mat_ptr
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_direction: Vec3 = unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    pub fn render(&self, world: &HittableList) {
        for j in (0..self.image_height).rev() {
            eprintln!("Scanlines remaining: {}", j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let u = (i as f64 + random_double()) / (self.image_width - 1) as f64;
                    let v = (j as f64 + random_double()) / (self.image_height - 1) as f64;
                    let r = self.get_ray(u, v);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }
                let _ = write_color(&mut std::io::stdout(), pixel_color, self.samples_per_pixel);
            }
        }
    }
}
