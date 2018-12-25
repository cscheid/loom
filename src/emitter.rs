use material::*;
use vector::Vec3;
use sampling;
use ray::Ray;
use hitable::*;

use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Emitter {
    pub emission: Vec3
}

// isotropic emitter
impl Material for Emitter {
    fn wants_importance_sampling(&self) -> bool { false }

    fn bsdf(&self, ray: &Ray, surface_normal: &Vec3) -> f64 {
        panic!("Should never call bsdf for emitter");
        0.0
    }

    fn albedo(&self, ray: &Ray, surface_normal: &Vec3) -> Vec3 {
        panic!("Should never call albedo for emitter");
        Vec3::new(0.0, 0.0, 0.0)
    }
    
    fn scatter(&self, _ray: &Ray, _rec: &HitRecord) -> Scatter
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


