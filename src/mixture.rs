use material::Material;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;

use rand;
use rand::Rng;
use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Mixture {
    mat_1: Rc<Material>,
    mat_2: Rc<Material>,
    u: f64
}

impl Material for Mixture {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() > self.u {
            self.mat_1.scatter(ray_in, rec, attenuation, scattered)
        } else {
            self.mat_2.scatter(ray_in, rec, attenuation, scattered)
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
