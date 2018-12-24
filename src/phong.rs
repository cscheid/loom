// "phong": this is just a disgusting approximation to a glossy material
// glossiness: 0 is metal
//             1 is ~lambertian but not really

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
        let r = vector::reflect(&vector::unit_vector(&ray_in.direction()), &rec.normal);

        // t is the normal of the disk from which we're sampling the
        // glossiness factor
        let t = Vec3::new(0.0, 0.0, 1.0);
        let g = sampling::random_in_unit_disk() * self.glossiness;
        let k = vector::cross(&r, &t);

        let theta = k.length();
        let kd = k * (1.0 / theta); // vector::unit_vector(k);
        let rotatedg = vector::rotate(&g, &kd, theta);

        let reflected = rotatedg + r;

        if reflected.dot(&rec.normal) > 0.0 {
            Scatter::Bounce(self.albedo, Ray::new(rec.p, reflected))
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
