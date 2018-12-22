use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Scatter {
        let reflected = vector::reflect(&vector::unit_vector(&ray_in.direction()), &rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Scatter::Bounce(self.albedo, scattered)
        } else {
            Scatter::Absorb
        }
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Metal {
    pub fn new(albedo: &Vec3) -> Rc<Material> {
        Rc::new(Metal {
            albedo: *albedo
        })
    }
}
