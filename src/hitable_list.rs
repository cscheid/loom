use aabb;
use aabb::AABB;
use hitable::*;
use ray::Ray;
use std::option::Option;

pub struct HitableList {
    pub v: Vec<Box<Hitable>>,
    pub bbox: Option<AABB>
}

impl HitableList {
    pub fn new(v: Vec<Box<Hitable>>) -> HitableList {
        let bbox = bounding_box_internal(&v);
        HitableList {
            v: v,
            bbox: bbox
        }
    }
}

fn bounding_box_internal(v: &Vec<Box<Hitable>>) -> Option<AABB> {
    let mut result = AABB::zero();
    for h in v.iter() {
        let bb_maybe = h.bounding_box();
        match bb_maybe {
            Some(bb) => result = aabb::surrounding_box(&result, &bb),
            None => return None
        };
    }
    Some(result)
}

impl Hitable for HitableList {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for hitable_box in self.v.iter() {
            match hitable_box.hit(r, t_min, closest_so_far) {
                None => {},
                Some(rec) => {
                    closest_so_far = rec.t;
                    hit_record = Some(rec);
                }
            }
        }
        hit_record
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.bbox
    }
}
