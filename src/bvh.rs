use hitable::*;
use aabb::AABB;
use aabb;
use ray::Ray;

use rand;
use rand::Rng;
use std::cmp::Ordering;

pub struct BVH {
    pub left: Box<Hitable>,
    pub right: Box<Hitable>,
    pub bbox: AABB
}

// yeah, this will do weird things with NaNs in the picture.
#[inline]
fn ffcmp(a: f64, b: f64) -> Ordering {
    if      a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else          { Ordering::Equal }
}

impl Hitable for BVH {
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(r, t_min, t_max) {
            let mut left_rec = HitRecord::new();
            let mut right_rec = HitRecord::new();
            let hit_left = self.left.hit(r, t_min, t_max, &mut left_rec);
            let hit_right = self.right.hit(r, t_min, t_max, &mut right_rec);
            if hit_left && hit_right {
                if left_rec.t < right_rec.t {
                    rec.set(&left_rec)
                } else {
                    rec.set(&right_rec)
                }
                true
            } else if hit_left {
                rec.set(&left_rec);
                true
            } else if hit_right {
                rec.set(&right_rec);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl BVH {
    pub fn build(objs: &mut Vec<Box<Hitable>>) -> Box<Hitable> {
        if objs.len() < 2 {
            panic!("Need nonempty objs!");
        } else if objs.len() == 1 {
            objs.remove(0)
        } else {
            let mut rng = rand::thread_rng();
            let axis = rng.gen_range(0, 3);
            // it'd be faster to _pivot by_, rather than sort by,
            // namely O(n log n) rather than O(n log n). but I don't
            // want to implement pivoting right now, and this will
            // be a minuscule fraction of the runtime.
            objs.sort_unstable_by(| h1, h2 | {
                let b1 = h1.bounding_box().unwrap();
                let b2 = h2.bounding_box().unwrap();
                ffcmp(b1.min()[axis], b2.min()[axis])
            });
            let median_ix = objs.len() / 2;
            let mut right_objs = objs.drain(median_ix..).collect();
            let mut left_objs = objs.drain(..).collect();
            let left_hitable = BVH::build(&mut left_objs);
            let right_hitable = BVH::build(&mut right_objs);
            let bbox = aabb::surrounding_box(
                &left_hitable.as_ref().bounding_box().unwrap(),
                &right_hitable.as_ref().bounding_box().unwrap());
            Box::new(BVH {
                left: left_hitable,
                right: right_hitable,
                bbox: bbox
            })
        }
    }
}
