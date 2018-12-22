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

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Scatter;
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl fmt::Debug for Material {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.debug(f)
    }
}
