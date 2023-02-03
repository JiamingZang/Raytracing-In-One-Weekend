use std::mem;

use crate::ray::Ray;
use crate::rtweekend::*;
use crate::vec3::*;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Point3,
    pub vertical: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub time0: f64,
    pub time1: f64,
}

// impl Default for Camera {
//     fn default() -> Self {
//         let aspect_ratio = 16.0 / 9.0;
//         let viewport_height = 2.0;
//         let viewport_width = aspect_ratio * viewport_height;
//         let focal_length = 1.0;

//         let origin = Point3::default();
//         let horizontal = Vec3::default().set_x(viewport_width);
//         let vertical = Vec3::default().set_y(viewport_height);
//         let lower_left_corner =
//             origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
//         Camera {
//             origin,
//             lower_left_corner,
//             horizontal,
//             vertical,
//         }
//     }
// }

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = cross(&vup, &w).unit_vector();
        let v = cross(&w, &u);

        // let focal_length = 1.0;

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;
        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn set_time(&self, time0: f64, time1: f64) -> Self {
        Camera {
            origin: self.origin,
            lower_left_corner: self.lower_left_corner,
            horizontal: self.horizontal,
            vertical: self.vertical,
            u: self.u,
            v: self.v,
            w: self.w,
            lens_radius: self.lens_radius,
            time0: time0,
            time1: time1,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::ray(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset,
            rand_double(self.time0, self.time1),
        )
    }
}
