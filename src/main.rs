mod aabb;
mod bvh;
mod camera;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use bvh::*;
use camera::*;
use hittable::*;
use hittable_list::*;
use moving_sphere::MovingSphere;
use ray::*;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use rtweekend::*;
use sphere::*;
use std::sync::Arc;
use vec3::*;

use crate::material::{Dielectric, Lambertian, Metal};

fn ray_color(r: &Ray, world: Arc<dyn Hittable + Send + Sync>, depth: i32) -> Color {
    let mut rec = HitRecord::default();

    if depth <= 0 {
        return Color::default();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::default(), Vec3::default(), 0.0);
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

fn random_scene() -> Arc<dyn Hittable + Send + Sync> {
    let mut world = HittableList::default();
    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::default().set_y(-1000.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_01();
            let center = Point3::new(a as f64 + 0.9 * rand_01(), 0.2, b as f64 + 0.9 * rand_01());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::rand_vec3_01() * Color::rand_vec3_01();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::default().set_y(rand_double(0.0, 0.5));

                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = Color::rand_vec3(0.5, 1.0);
                    let fuzz = rand_double(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }

        let material1 = Arc::new(Dielectric::new(1.5));
        world.add(Arc::new(Sphere::new(
            Point3::default().set_y(1.0),
            1.0,
            material1,
        )));

        let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        world.add(Arc::new(Sphere::new(
            Point3::default().set_y(1.0).set_x(-4.0),
            1.0,
            material2,
        )));

        let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
        world.add(Arc::new(Sphere::new(
            Point3::default().set_y(1.0).set_x(4.0),
            1.0,
            material3,
        )));
    }

    Arc::new(BVHNode::new(&world, 0.0, 1.0))
}

fn main() {
    //Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World

    // let mut world = HittableList::default();
    // let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));
    // world.add(Rc::new(Sphere::new(
    //     Point3::default().set_z(-1.0),
    //     0.5,
    //     material_center,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left.clone(),
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     -0.45,
    //     material_left,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    let world = random_scene();
    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::default();
    let vup = Vec3::default().set_y(1.0);
    let disk_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        disk_to_focus,
    )
    .set_time(0.0, 1.0);
    //Render
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..(image_height - 1) {
        let j = image_height - j;
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let pixel_color = (0..samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let u = (i as f64 + rand_01()) / (image_width as f64 - 1.0);
                    let v = (j as f64 + rand_01()) / (image_height as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    ray_color(&r, world.clone(), max_depth)
                })
                .collect::<Vec<Color>>()
                .iter()
                .fold(Color::default(), |a: Color, b: &Color| a + *b);
            // for _ in 0..samples_per_pixel {
            //     let u = (i as f64 + rand_01()) / (image_width as f64 - 1.0);
            //     let v = (j as f64 + rand_01()) / (image_height as f64 - 1.0);
            //     let r = cam.get_ray(u, v);
            //     pixel_color += ray_color(&r, &world, max_depth);
            // }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone.\n");
}
