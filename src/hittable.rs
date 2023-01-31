use std::rc::Rc;

use crate::{
    material::{Lambertian, Material},
    ray::*,
    vec3::*,
};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl Default for HitRecord {
    fn default() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: true,
            mat_ptr: Rc::new(Lambertian::new(Color::default())),
        }
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
