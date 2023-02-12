use std::{mem, sync::Arc};

use crate::{aabb::*, hittable::*, vec3::Point3};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
}

impl HittableList {
    pub fn new(object: Arc<dyn Hittable + Sync + Send>) -> HittableList {
        let mut list = HittableList::default();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in &self.objects {
            if object.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                // rec.p = temp_rec.p;
                // rec.t = temp_rec.t;
                // rec.front_face = temp_rec.front_face;
                // rec.normal = temp_rec.normal;
                *rec = mem::take(&mut temp_rec);
            }
        }

        hit_anything
    }

    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut crate::aabb::Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut temp_box = Aabb::new(Point3::default(), Point3::default());
        let mut first_box = true;

        for object in self.objects.clone() {
            if !object.bounding_box(time0, time1, &mut temp_box) {
                return false;
            }
            *output_box = if first_box {
                temp_box
            } else {
                surrounding_box(*output_box, temp_box)
            };
            first_box = false;
        }

        true
    }
}
