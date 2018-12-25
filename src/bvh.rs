use aabb::AABB;
use aabb;
use hitable::*;
use hitable_list::*;
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
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        if self.bbox.hit(r, t_min, t_max) {
            // let mut left_rec = HitRecord::new();
            // let mut right_rec = HitRecord::new();
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            
            match (hit_left, hit_right) {
                (Some(left_rec), Some(right_rec)) => {
                    if left_rec.t < right_rec.t {
                        Some(left_rec)
                    } else {
                        Some(right_rec)
                    }
                },
                (Some(left_rec), None) => Some(left_rec),
                (None, Some(right_rec)) => Some(right_rec),
                (None, None) => None
            }
        } else {
            None
        }
    }
    // this should never be called
    fn importance_distribution(&self) -> Option<AABB> {
        panic!("importance_distribution called on BVH!");
        None
    }
}

// profiled on a relatively small scene - should probably do this more carefully.
const MIN_LENGTH: usize = 32;

impl BVH {
   
    pub fn build(mut objs: Vec<Box<Hitable + Send + Sync>>) -> Box<Hitable + Send + Sync> {
        if objs.len() == 0 {
            panic!("Need nonempty objs!");
        } else if objs.len() <= MIN_LENGTH {
            Box::new(HitableList::new(objs))
            // let result = objs.remove(0);
            // // eprintln!("leaf, bounding box: {:?}", result.bounding_box());
            // result
        } else {
            let mut rng = rand::thread_rng();
            let axis = rng.gen_range(0, 3);
            // eprintln!("{} nodes, Splitting on axis {}", objs.len(), axis);
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
            let right_objs = objs.drain(median_ix..).collect();
            let left_objs = objs.drain(..).collect();
            let left_hitable = BVH::build(left_objs);
            let right_hitable = BVH::build(right_objs);
            let bbox = aabb::surrounding_box(
                &left_hitable.as_ref().bounding_box().unwrap(),
                &right_hitable.as_ref().bounding_box().unwrap());
            // eprintln!("Bounding box: {:?}", bbox);
            Box::new(BVH {
                left: left_hitable,
                right: right_hitable,
                bbox: bbox
            })
        }
    }
}
