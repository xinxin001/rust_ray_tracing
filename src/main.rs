mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::rc::Rc;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::Lambertian,
    ray::Ray,
    sphere::Sphere,
    vec3::{dot, Color, Point3},
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

fn main() -> std::io::Result<()> {
    // World
    let mut world = HittableList::new();

    let r = f64::cos(rtweekend::PI / 4.);
    let material_left = Rc::new(Lambertian::new(Color::new(0., 0., 1.)));
    let material_right = Rc::new(Lambertian::new(Color::new(1., 0., 0.)));

    world.add(Box::new(Sphere::with_values(
        Point3::new(-r, 0., -1.),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(r, 0., -1.),
        r,
        material_right,
    )));
    // Camera
    let camera = Camera::new(10, 16. / 9., 400, 10, 90.);

    println!("P3\n{} {}\n255", camera.image_width, camera.image_height);
    camera.render(&world);
    eprint!("Done.");
    Ok(())
}
