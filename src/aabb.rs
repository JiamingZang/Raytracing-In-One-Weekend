use std::mem::swap;

use crate::{ray::Ray, vec3::Point3};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn new(minimum: Point3, maximum: Point3) -> Aabb {
        Aabb { minimum, maximum }
    }

    // pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
    //     for a in 0..3 {
    //         let t0 = f64::min(
    //             self.minimum[a] - r.origin()[a] / r.direction()[a],
    //             self.maximum[a] - r.origin()[a] / r.direction()[a],
    //         );
    //         let t1 = f64::min(
    //             self.minimum[a] - r.origin()[a] / r.direction()[a],
    //             self.maximum[a] - r.origin()[a] / r.direction()[a],
    //         );
    //         t_min = f64::max(t0, t_min);
    //         t_max = f64::min(t1, t_max);
    //         if t_max <= t_min {
    //             return false;
    //         }
    //     }
    //     true
    // }

    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.minimum[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.maximum[a] - r.origin()[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = f64::max(t0, t_min);
            t_max = f64::min(t1, t_max);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: Aabb, box1: Aabb) -> Aabb {
    let small = Point3::new(
        f64::min(box0.minimum.x, box1.minimum.x),
        f64::min(box0.minimum.y, box1.minimum.y),
        f64::min(box0.minimum.z, box1.minimum.z),
    );

    let big = Point3::new(
        f64::max(box0.maximum.x, box1.maximum.x),
        f64::max(box0.maximum.y, box1.maximum.y),
        f64::max(box0.maximum.z, box1.maximum.z),
    );

    Aabb {
        minimum: small,
        maximum: big,
    }
}
