use material::*;
use vector::Vec3;
use vector;
use sampling;
use ray::Ray;
use hitable::*;

use std::fmt;
use std::fmt::Debug;
use std::f64;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

// two-sided lambertian
impl Material for Lambertian {
    fn wants_importance_sampling(&self) -> bool { false }
    fn albedo(&self, ray: &Ray, surface_normal: &Vec3) -> Vec3 {
        self.albedo
    }
    
    fn bsdf(&self, ray: &Ray, surface_normal: &Vec3) -> f64 {
        let x = vector::unit_vector(&ray.direction()).dot(surface_normal);
        if x <= 0.0 {
            0.0
        } else {
            x / std::f64::consts::PI
        }
    }

    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Scatter {

        // this is a 1-sample monte-carlo approximation of
        // the integral of the resulting radiance
        // over all outgoing directions multiplied by the cosine of the
        // angle between the direction and the normal. In other words,
        // `target` is drawn with probability \propto cos(dir, normal)
        // Equivalently, this is the integral of radiance(dir) * cos(dir, normal) over all dirs
        //
        // in order to 
        
        let target;
        if rec.normal.dot(&ray.direction()) > 0.0 {
            target = rec.p - rec.normal + sampling::random_in_unit_sphere();
        } else {
            target = rec.p + rec.normal + sampling::random_in_unit_sphere();
        }
        Scatter::Bounce(self.albedo, Ray::new(rec.p, target - rec.p))
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }

    fn is_emitter(&self) -> bool { false }
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Box<Material> {
        Box::new(Lambertian {
            albedo: *albedo
        })
    }
}
