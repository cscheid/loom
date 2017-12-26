use material::Material;
use vector::Vec3;
use ray::Ray;
use hitable::*;
use random::*;
use serializable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;
use serde_json::{Map, Value};

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Mixture {
    mat_1: Rc<Material>,
    mat_2: Rc<Material>,
    u: f64
}

impl Material for Mixture {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        if rand_double() > self.u {
            self.mat_1.scatter(ray_in, rec, attenuation, scattered)
        } else {
            self.mat_2.scatter(ray_in, rec, attenuation, scattered)
        }
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    // fn to_json(&self) -> Value {
    //     let mut m = Map::new();
    //     m.insert("mat_1".to_string(), self.mat_1.to_json());
    //     m.insert("mat_2".to_string(), self.mat_2.to_json());
    //     m.insert("u".to_string(), self.u.into());
    //     tagged_object("mixture".to_string(), m)
    // }
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
