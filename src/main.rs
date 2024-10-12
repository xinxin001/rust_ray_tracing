mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::sync::Arc;

use crate::{
    camera::Camera,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    ray::Ray,
    rtweekend::{random_double, random_double_range},
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

    // let material_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // world.add(Box::new(Sphere::with_values(
    //     Point3::new(-4.0, -1000., 0.0),
    //     1000.0,
    //     material_ground.clone(),
    // )));

    let material_test = Arc::new(Lambertian::new(Color::new(1., 0., 0.)));
    world.add(Box::new(Sphere::with_values(
        Point3::new(0., 0., -3.),
        1.,
        material_test.clone(),
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );
            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = random_double_range(0., 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere::with_values(center, 0.2, sphere_material)));
            }
        }
    }
    // Camera

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble.clone(),
    )));
    world.add(Box::new(Sphere::with_values(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));
    let camera = Camera::new(
        50,
        16. / 9.,
        400,
        50,
        90.,
        Point3::new(0., 0., 0.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        1.,
        0.6,
    );

    println!("P3\n{} {}\n255", camera.image_width, camera.image_height);
    camera.render(&world);
    eprint!("Done.");
    Ok(())
}
