use material::*;
use vector::Vec3;
use ray::Ray;
use hitable::*;
use random::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Mixture {
    mat_1: Rc<Material>,
    mat_2: Rc<Material>,
    u: f64
}

impl Material for Mixture {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Scatter {
        if rand_double() > self.u {
            self.mat_1.scatter(ray_in, rec)
        } else {
            self.mat_2.scatter(ray_in, rec)
        }
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Mixture {
    pub fn new(mat_1: Rc<Material>,
               mat_2: Rc<Material>,
               u: f64) -> Rc<Material> {
        Rc::new(Mixture {
            mat_1: mat_1,
            mat_2: mat_2,
            u: u
        })
    }
}
