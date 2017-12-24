use vector::*;
use ray::*;

#[derive(Copy, Debug, Clone)]
pub struct AABB {
    pub _min: Vec3,
    pub _max: Vec3
}

fn ffmin(a: f64, b: f64) -> f64 { if a < b { a } else { b } }
fn ffmax(a: f64, b: f64) -> f64 { if a > b { a } else { b } }

impl AABB {
    pub fn zero() -> AABB {
        AABB {
            _min: Vec3::new( 1e20,  1e20,  1e20),
            _max: Vec3::new(-1e20, -1e20, -1e20)
        }
    }
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB {
            _min: min,
            _max: max
        }
    }

    pub fn min(&self) -> Vec3 {
        self._min
    }
    pub fn max(&self) -> Vec3 {
        self._max
    }

    pub fn hit(&self, r: &Ray, _tmin: f64, _tmax: f64) -> bool {
        let mut tmin = _tmin;
        let mut tmax = _tmax;
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let t0 = ffmin((self._min[a] - r.origin()[a]) * inv_d,
                           (self._max[a] - r.origin()[a]) * inv_d);
            let t1 = ffmax((self._min[a] - r.origin()[a]) * inv_d,
                           (self._max[a] - r.origin()[a]) * inv_d);
            tmin = ffmax(t0, tmin);
            tmax = ffmin(t1, tmax);
            if tmax <= tmin {
                return false;
            }
        }
        true
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = Vec3::new(ffmin(box0.min()[0], box1.min()[0]),
                          ffmin(box0.min()[1], box1.min()[1]),
                          ffmin(box0.min()[2], box1.min()[2]));
    let big   = Vec3::new(ffmax(box0.max()[0], box1.max()[0]),
                          ffmax(box0.max()[1], box1.max()[1]),
                          ffmax(box0.max()[2], box1.max()[2]));
    AABB::new(small, big)
}
