use material::*;
use vector::Vec3;
use sampling;
use ray::Ray;
use hitable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Emitter {
    pub emission: Vec3
}

// isotropic emitter
impl Material for Emitter {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Scatter
    {
        Scatter::Emit(self.emission)
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Emitter {
    pub fn new(emission: &Vec3) -> Rc<Material> {
        Rc::new(Emitter {
            emission: *emission
        })
    }
}


