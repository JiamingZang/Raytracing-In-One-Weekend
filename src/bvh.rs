use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::{surrounding_box, AABB},
    hittable::Hittable,
    hittable_list::HittableList,
    rtweekend::rand_int,
    vec3::Vec3,
};

#[derive(Clone)]
pub struct BVHNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    r#box: AABB,
}

impl BVHNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Self {
        let result = BVHNode {
            left: list.objects[0].clone(),
            right: list.objects[0].clone(),
            r#box: AABB {
                minimum: Vec3::default(),
                maximum: Vec3::default(),
            },
        };
        Self::bvh_node(
            result,
            list.objects.clone(),
            0,
            list.objects.len(),
            time0,
            time1,
        )
    }

    pub fn bvh_node(
        mut node: BVHNode,
        mut objects: Vec<Arc<dyn Hittable + Send + Sync>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let axis = rand_int(0, 2);
        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let object_pan = end - start;
        match object_pan {
            1 => {
                node.left = objects[start].clone();
                node.right = objects[start].clone();
            }
            2 => match comparator(objects[start].clone(), objects[start + 1].clone()) {
                Ordering::Less => {
                    node.left = objects[start].clone();
                    node.right = objects[start + 1].clone();
                }
                Ordering::Greater => {
                    node.left = objects[start + 1].clone();
                    node.right = objects[start].clone();
                }
                _ => {}
            },
            _ => {
                let mut tempvec = vec![];
                for i in start..end {
                    tempvec.push(objects[i].clone());
                }
                tempvec.sort_by(|a, b| comparator(a.clone(), b.clone()));
                for i in start..end {
                    objects[i] = tempvec[i - start].clone();
                }

                let mid = start + object_pan / 2;
                node.left = Arc::new(BVHNode::bvh_node(
                    node.clone(),
                    objects.clone(),
                    start,
                    mid,
                    time0,
                    time1,
                ))
                .clone();
                node.right = Arc::new(BVHNode::bvh_node(
                    node.clone(),
                    objects.clone(),
                    mid,
                    end,
                    time0,
                    time1,
                ));
            }
        }

        let mut box_left = AABB::new(Vec3::default(), Vec3::default());
        let mut box_right = AABB::new(Vec3::default(), Vec3::default());
        if !node.left.bounding_box(time0, time1, &mut box_left)
            || !node.right.bounding_box(time0, time1, &mut box_right)
        {
            eprintln!("no bounding box in bvh_node constructor")
        }
        node.r#box = surrounding_box(box_left, box_right);
        node
    }

    pub fn box_compare(
        a: Arc<dyn Hittable + Send + Sync>,
        b: Arc<dyn Hittable + Send + Sync>,
        axis: i32,
    ) -> Ordering {
        let mut box_a = AABB::new(Vec3::default(), Vec3::default());
        let mut box_b = AABB::new(Vec3::default(), Vec3::default());
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            eprintln!("No bounding box in bvh_node constructor.");
        };
        if box_a.minimum[axis] < box_b.minimum[axis] {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    pub fn box_x_compare(
        a: Arc<dyn Hittable + Send + Sync>,
        b: Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    pub fn box_y_compare(
        a: Arc<dyn Hittable + Send + Sync>,
        b: Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    pub fn box_z_compare(
        a: Arc<dyn Hittable + Send + Sync>,
        b: Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        if !self.r#box.hit(r, t_min, t_max) {
            return false;
        };
        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);
        return hit_right || hit_left;
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut AABB) -> bool {
        *output_box = self.r#box;
        true
    }
}
