use vector::*;
use ray::*;
use disc::*;

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
    pub fn from_point(p: Vec3) -> AABB {
        AABB {
            _min: p,
            _max: p
        }
    }
    pub fn from_points(ps: &[Vec3]) -> AABB {
        let mut result = AABB::zero();
        for p in ps {
            result.update(p);
        }
        result
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
    pub fn update(&mut self, p: &Vec3) {
        self._min[0] = ffmin(self._min[0], p[0]);
        self._min[1] = ffmin(self._min[1], p[1]);
        self._min[2] = ffmin(self._min[2], p[2]);
        self._max[0] = ffmax(self._max[0], p[0]);
        self._max[1] = ffmax(self._max[1], p[1]);
        self._max[2] = ffmax(self._max[2], p[2]);
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
            if tmax < tmin {
                return false;
            }
        }
        true
    }

    // far from best possible disc, but eh
    pub fn project_to_disc_on_sphere(&self, center: &Vec3) -> Disc {
        let mut pts = vec![
            Vec3::new(self._min[0], self._min[1], self._min[2]),
            Vec3::new(self._min[0], self._min[1], self._max[2]),
            Vec3::new(self._min[0], self._max[1], self._min[2]),
            Vec3::new(self._min[0], self._max[1], self._max[2]),
            Vec3::new(self._max[0], self._min[1], self._min[2]),
            Vec3::new(self._max[0], self._min[1], self._max[2]),
            Vec3::new(self._max[0], self._max[1], self._min[2]),
            Vec3::new(self._max[0], self._max[1], self._max[2])];

        let mut average = Vec3::new(0.0, 0.0, 0.0);
        for i in 0..8 {
            let p_v = *center + unit_vector(&(pts[i] - *center));
            pts[i] = p_v;
            average = average + p_v;
        }
        average = average / 8.0;
        let mut max_dist = 0.0;
        for i in 0..8 {
            let d = (average - pts[i]).length();
            if d > max_dist {
                max_dist = d;
            }
        }
        return Disc::new(average, unit_vector(&(*center - average)), max_dist);
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


#[test]
fn it_works()
{
    let mut box1 = AABB::from_point(Vec3::new(1.0, 1.0, 1.0));
    box1.update(&Vec3::new(2.0, 2.0, 2.0));

    assert!(box1.hit(&Ray::new(Vec3::new(1.5, 1.5, 0.0),
                               Vec3::new(0.0, 0.0, 1.0)),
                     0.0001, 1e20));

    assert!(!box1.hit(&Ray::new(Vec3::new(1.5, 1.5, 0.0),
                                Vec3::new(0.0, 0.0, 1.0)),
                      0.0001, 0.5));
    assert!(!box1.hit(&Ray::new(Vec3::new(1.5, 1.5, 0.0),
                                Vec3::new(1.6, 0.0, 1.0)),
                      0.0001, 1e20));
}
