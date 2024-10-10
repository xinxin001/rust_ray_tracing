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
    material::{Dielectric, Lambertian, Metal},
    ray::Ray,
    sphere::Sphere,
    vec3::{dot, Color, Point3, Vec3},
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

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_bubble = Rc::new(Dielectric::new(1. / 1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.));

    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, 0.0, -2.),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(-1.0, 0.0, -1.2),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(-1., 0., -1.2),
        0.4,
        material_bubble.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(1.0, 0.0, -0.8),
        0.5,
        material_right.clone(),
    )));

    // Camera
    let camera = Camera::new(
        10,
        16. / 9.,
        400,
        10,
        90.,
        Point3::new(0., 0., 0.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        2.,
        10.,
    );

    println!("P3\n{} {}\n255", camera.image_width, camera.image_height);
    camera.render(&world);
    eprint!("Done.");
    Ok(())
}
