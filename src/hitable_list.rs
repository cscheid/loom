use aabb;
use aabb::AABB;
use hitable::*;
use ray::Ray;
use std::rc::Rc;
use std::option::Option;

pub struct HitableList {
    pub v: Vec<Box<Hitable>>
}

impl HitableList {
    pub fn new(v: Vec<Box<Hitable>>) -> HitableList {
        HitableList { v: v }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for hitable_box in self.v.iter() {
            let hitable = &*hitable_box;
            if hitable.hit(r, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                rec.t = temp_record.t;
                rec.p = temp_record.p;
                rec.normal = temp_record.normal;
                rec.material = Some(Rc::clone(&temp_record.material.as_ref().expect("")));
            }
        }
        return hit_anything;
    }

    fn bounding_box(&self) -> Option<AABB> {
        let mut result = AABB::zero();
        for h in self.v.iter() {
            let bb_maybe = h.bounding_box();
            match bb_maybe {
                Some(bb) => result = aabb::surrounding_box(&result, &bb),
                None => return None
            };
        }
        Some(result)
    }
}
