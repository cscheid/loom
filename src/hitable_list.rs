use hitable::*;
use ray::Ray;
use std::rc::Rc;

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
}
