use ray::Ray;
use vector::Vec3;
use hitable::HitRecord;
use std::fmt;

//////////////////////////////////////////////////////////////////////////////

pub enum Scatter {
    Bounce(Vec3, Ray),
    Emit(Vec3),
    Absorb
}

pub trait Material: Send + Sync {
    fn wants_importance_sampling(&self) -> bool;
    fn albedo(&self, ray: &Ray, surface_normal: &Vec3) -> Vec3;
    fn bsdf(&self, ray: &Ray, surface_normal: &Vec3) -> f64;
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Scatter;
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn is_emitter(&self) -> bool;
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug(f)
    }
}
