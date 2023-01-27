use std::{mem, rc::Rc};

use crate::hittable::*;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl Default for HittableList {
    fn default() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }
}

impl HittableList {
    pub fn new(object: Rc<dyn Hittable>) -> HittableList {
        let mut list = HittableList::default();
        list.add(object);
        list
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object.into());
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
}
