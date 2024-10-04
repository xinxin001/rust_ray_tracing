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
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel to below
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub image_height: usize,
    samples_per_pixel: u32,
    max_depth: u32,
}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: u32) -> Self {
        // Image
        let max_depth = 10;

        let mut image_height: usize = (image_width as f64 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = (image_width / image_height) as f64 * viewport_height;

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., viewport_height, 0.);

        let center = Point3::new(0.0, 0.0, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left =
            center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;
        Self {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at a randomly sampled
        // point around the pixel location i, j
        let offset: Vec3 = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(self.center, ray_direction);
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
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }
                let _ = write_color(&mut std::io::stdout(), pixel_color, self.samples_per_pixel);
            }
        }
    }
}
