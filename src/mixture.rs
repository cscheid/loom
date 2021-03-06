use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;
use random::*;

use std::fmt;
use std::fmt::Debug;

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Mixture {
    mat_1: Box<Material>,
    mat_2: Box<Material>,
    u: f64
}

impl Material for Mixture {
    fn wants_importance_sampling(&self) -> bool {
        self.mat_1.wants_importance_sampling() &&
            self.mat_2.wants_importance_sampling()
    }

    fn bsdf(&self, ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> f64 {
        (1.0 - self.u) * &self.mat_1.bsdf(ray_in, ray_out, surface_normal) +
            self.u * &self.mat_2.bsdf(ray_in, ray_out, surface_normal)
    }
    
    fn albedo(&self, ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> Vec3 {
        vector::lerp(&self.mat_1.albedo(ray_in, ray_out, surface_normal),
                     &self.mat_2.albedo(ray_in, ray_out, surface_normal),
                     self.u)
    }

    
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Scatter {
        if rand_double() < self.u {
            self.mat_1.scatter(ray_in, rec)
        } else {
            self.mat_2.scatter(ray_in, rec)
        }
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    fn is_emitter(&self) -> bool {
        self.mat_1.is_emitter() || self.mat_2.is_emitter()
    }
}

impl Mixture {
    pub fn new(mat_1: Box<Material>,
               mat_2: Box<Material>,
               u: f64) -> Box<Material> {
        Box::new(Mixture {
            mat_1: mat_1,
            mat_2: mat_2,
            u: u
        })
    }
}
