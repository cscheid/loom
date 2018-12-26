use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;

use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    fn wants_importance_sampling(&self) -> bool { false }

    // in truth it's a dirac delta but this is correct
    // in all but a measure-zero set. effectively it
    // means that importance sampling from the lights
    // is useless
    fn bsdf(&self, _ray_in: &Ray, _ray_out: &Ray, _surface_normal: &Vec3) -> f64 {
        0.0
    }
    fn albedo(&self, _ray: &Ray, _ray_out: &Ray, _surface_normal: &Vec3) -> Vec3 {
        self.albedo
    }
    
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

    fn is_emitter(&self) -> bool { false }
}

impl Metal {
    pub fn new(albedo: &Vec3) -> Box<Material> {
        Box::new(Metal {
            albedo: *albedo
        })
    }
}
