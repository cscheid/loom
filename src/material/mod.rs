pub mod lambertian;
pub mod metal;
pub mod dielectric;
pub mod mixture;

use ray::Ray;
use vector::Vec3;
use hitable::HitRecord;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////
    
pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool;

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug(f)
    }
}
