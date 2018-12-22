use material::*;
use vector::Vec3;
use sampling;
use ray::Ray;
use hitable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

// two-sided lambertian
impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Scatter {
        let target;
        if rec.normal.dot(&_ray.direction()) > 0.0 {
            target = rec.p - rec.normal + sampling::random_in_unit_sphere();
        } else {
            target = rec.p + rec.normal + sampling::random_in_unit_sphere();
        }
        Scatter::Bounce(self.albedo, Ray::new(rec.p, target - rec.p))
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Rc<Material> {
        Rc::new(Lambertian {
            albedo: *albedo
        })
    }
}
