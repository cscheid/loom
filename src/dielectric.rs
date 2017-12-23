use material::Material;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;

use std::rc::Rc;
use std::fmt;
use std::fmt::Debug;
use rand;
use rand::Rng;

#[derive(Debug)]
pub struct Dielectric {
    refraction_index: f64
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0s = r0 * r0;
    r0s + (1.0 - r0s) * (1.0 - cosine).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord,
               attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let dot = r_in.direction().dot(&rec.normal);
        let mut outward_normal = Vec3::zero();
        let reflected = vector::reflect(&r_in.direction(), &rec.normal);
        let mut ni_over_nt = 0.0;
        let mut cosine = 0.0;
        
        attenuation.set(&Vec3::new(1.0, 1.0, 1.0));
        if dot > 0.0 {
            outward_normal = -rec.normal;
            ni_over_nt = self.refraction_index;
            cosine = self.refraction_index * dot / r_in.direction().length();
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.refraction_index;
            cosine = -dot / r_in.direction().length();
        }
        match vector::refract(&r_in.direction(), &outward_normal, ni_over_nt) {
            Some(refracted) => {
                let reflected_prob = schlick(cosine, self.refraction_index); 
                let mut rng = rand::thread_rng();
                if rng.gen::<f64>() < reflected_prob {
                    scattered.set(&rec.p, &reflected);
                } else {
                    scattered.set(&rec.p, &refracted);
                }
            },
            None => {
                    scattered.set(&rec.p, &reflected);
            }
        }
        true
    }
    
    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}


impl Dielectric {
    pub fn new(refraction_index: f64) -> Rc<Material> {
        Rc::new(Dielectric {
            refraction_index: refraction_index
        })
    }
}
