mod camera;
mod color;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use rtweekend::INFINITY;
use vec3::{random_in_hemisphere, random_in_unit_sphere, random_unit_vector};

use crate::{
    camera::Camera,
    color::write_color,
    hittable_list::HittableList,
    ray::Ray,
    rtweekend::random_double,
    sphere::Sphere,
    vec3::{dot, unit_vector, Color, Point3, Vec3},
};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = r.direction().length_squared();
    let half_b = dot(oc, r.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    }
    return (-half_b - discriminant.sqrt()) / a;
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut rec = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target: Point3 = rec.p + random_in_hemisphere(rec.normal);
        return ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1) * 0.5;
    }
    let unit_direction: Vec3 = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let camera = Camera::new();

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(&mut std::io::stdout(), pixel_color, samples_per_pixel)?;
        }
    }
    eprint!("Done.");
    Ok(())
}
