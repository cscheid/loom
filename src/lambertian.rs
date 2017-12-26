use material::Material;
use vector::Vec3;
use sampling;
use ray::Ray;
use hitable::*;
use serializable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;
use serde_json::{Map, Value};

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let target = rec.p + rec.normal + sampling::random_in_unit_sphere();
        scattered.set(&rec.p, &(target - rec.p));
        attenuation.set(&self.albedo);
        true
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    // fn to_json(&self) -> Value {
    //     let mut m = Map::new();
    //     m.insert("albedo".to_string(), self.albedo);
    //     tagged_object("lambertian".to_string(), m)
    // }
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Rc<Material> {
        Rc::new(Lambertian {
            albedo: *albedo
        })
    }
}
