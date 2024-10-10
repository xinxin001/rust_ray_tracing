use crate::{
    color::write_color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
    rtweekend::{degrees_to_radians, random_double, INFINITY},
    vec3::{cross, random_in_unit_disk, unit_vector, Color, Point3, Vec3},
};

pub struct Camera {
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel 0,0
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel to below
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    samples_per_pixel: u32,
    pixel_samples_scale: f64,
    max_depth: u32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub defocus_disk_u: Vec3,
    pub defocus_disk_v: Vec3,
}

fn sample_square() -> Vec3 {
    Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.)
}

impl Camera {
    pub fn new(
        max_depth: u32,
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        vfov: f64,
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        focus_dist: f64,
        defocus_angle: f64,
    ) -> Self {
        let mut image_height = (image_width as f64 / aspect_ratio) as u32;
        image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        // Determine viewport dimensions
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.);
        let viewport_height = 2. * h * focus_dist;
        let viewport_width = (image_width as f64 / image_height as f64) * viewport_height;

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        let w = lookfrom - lookat;
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let center = lookfrom;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel
        let viewport_upper_left = center - (w * focus_dist) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        Self {
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            aspect_ratio,
            image_width,
            image_height,
            samples_per_pixel,
            pixel_samples_scale,
            max_depth,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }

    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        // Construct a camera ray originating from the origin and directed at a randomly sampled
        // point around the pixel location i, j
        let offset: Vec3 = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (i + offset.x()))
            + (self.pixel_delta_v * (j + offset.y()));
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction);
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
        for j in 0..self.image_height {
            eprintln!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_color += self.ray_color(&r, world, self.max_depth);
                }
                let _ = write_color(
                    &mut std::io::stdout(),
                    pixel_color * self.pixel_samples_scale,
                );
            }
        }
    }
}
