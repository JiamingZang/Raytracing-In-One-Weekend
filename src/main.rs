mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;
use std::rc::Rc;
mod material;

use camera::*;
use hittable::*;
use hittable_list::*;
use ray::*;
use rtweekend::*;
use sphere::*;
use vec3::*;

use crate::material::{Dielectric, Lambertian, Metal};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::ray(Vec3::default(), Vec3::default());
        let mut attenuation = Color::default();
        if rec
            .mat_ptr
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    let mut world = HittableList::default();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::default().set_z(-1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let cam = Camera::new(
        Point3::new(-2.0, 2.0, 1.0),
        Point3::default().set_z(-1.0),
        Vec3::default().set_y(1.0),
        20.0,
        aspect_ratio,
    );
    //Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..(image_height - 1) {
        let j = image_height - j;
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand_01()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rand_01()) / (image_height as f64 - 1.0);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.\n");
}
