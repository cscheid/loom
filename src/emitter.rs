use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;

use std::fmt;
use std::fmt::Debug;

// testing imports
#[allow(unused_imports)]
use sampling;

#[derive(Debug)]
pub struct Emitter {
    pub emission: Vec3
}

// isotropic emitter
impl Material for Emitter {
    fn wants_importance_sampling(&self) -> bool { false }

    fn bsdf(&self, _ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> f64 {
        let x = vector::unit_vector(&ray_out.direction()).dot(surface_normal);
        if x <= 0.0 {
            0.0
        } else {
            2.0 * x
        }
    }

    fn albedo(&self, _ray_in: &Ray, _ray_out: &Ray, _surface_normal: &Vec3) -> Vec3 {
        panic!("Should never call albedo for emitter")
    }
    
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord) -> Scatter
    {
        Scatter::Emit(self.emission)
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    fn is_emitter(&self) -> bool { true }
}

impl Emitter {
    pub fn new(emission: &Vec3) -> Box<Material> {
        Box::new(Emitter {
            emission: *emission
        })
    }
}


