use material::Material;
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
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = vector::reflect(&vector::unit_vector(&ray_in.direction()), &rec.normal);
        scattered.set(&rec.p, &reflected);
        attenuation.set(&self.albedo);
        scattered.direction().dot(&rec.normal) > 0.0
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
