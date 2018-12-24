// "phong": this is just a disgusting approximation to a glossy material
// glossiness: 0 is metal
//             1 is lambertian

use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;
use sampling;

use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Phong {
    albedo: Vec3,
    glossiness: f64
}

impl Material for Phong {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Scatter {
        // lambertian
        let lambertian_target;
        if rec.normal.dot(&ray_in.direction()) > 0.0 {
            lambertian_target = rec.p - rec.normal + sampling::random_in_unit_sphere();
        } else {
            lambertian_target = rec.p + rec.normal + sampling::random_in_unit_sphere();
        }

        // metal
        let metal_target = vector::reflect(&vector::unit_vector(&ray_in.direction()), &rec.normal);

        // yay for hacks: glossy: lerp(lambertian, reflected, glossiness)
        let scattered = vector::lerp(&metal_target,
                                     &lambertian_target, self.glossiness);

        if scattered.dot(&rec.normal) > 0.0 {
            Scatter::Bounce(self.albedo, Ray::new(rec.p, scattered))
        } else {
            Scatter::Absorb
        }
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}

impl Phong {
    pub fn new(albedo: &Vec3, glossiness: f64) -> Box<Material> {
        Box::new(Phong {
            albedo: *albedo,
            glossiness: glossiness
        })
    }
}
