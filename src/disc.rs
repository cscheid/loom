use vector::Vec3;
use vector;
use plane::Plane;
use ray::Ray;
use sphere_geometry::SphereGeom;
use sampling;

use std::f64;
// a disc is an intersection of a plane and a sphere

#[derive(Debug)]
pub struct Disc {
    pub center: Vec3,
    pub normal: Vec3,
    pub radius: f64,
}

pub enum Visibility {
    Full,
    Partial,
    Hidden
}

impl Disc {
    pub fn plane(&self) -> Plane {
        Plane::new(vector::dot(&self.center, &self.normal),
                   &self.normal)
    }

    pub fn normal_space(&self) -> (Vec3, Vec3) {
        let mut u_vec = vector::cross(&self.normal, &Vec3::new(1.0, 0.0, 0.0));
        if u_vec.length() == 0.0 {
            u_vec = vector::cross(&self.normal, &Vec3::new(0.0, 1.0, 0.0));
        }
        (u_vec, vector::cross(&self.normal, &u_vec))
    }

    pub fn random(&self) -> Vec3 {
        let v = sampling::random_in_unit_disk();
        self.sample(v.x(), v.y())
    }
    
    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        let n = self.normal_space();
        let u_vec = n.0;
        let v_vec = n.1;
        self.center + u_vec * u + v_vec * v
    }
    
    pub fn sphere(&self) -> SphereGeom {
        SphereGeom::new(self.center, self.radius)
    }

    pub fn new(center: Vec3, normal: Vec3, radius: f64) -> Disc {
        Disc { center: center, normal: normal, radius: radius }
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<Vec3>
    {
        self.plane()
            .intersect_ray(&ray)
            .and_then(|p| {
                if (p - self.center).length() > self.radius {
                    None
                } else {
                    Some(p)
                }
            })
    }

    // returns a 1-sample MC estimation of the subtended
    // (clipped) angle of the disc in the hemisphere
    pub fn hemi_disc_subtended_angle(&self, disc: &Disc) -> (f64, Vec3) {
        let disc_sample = disc.random();
        let sample_direction = vector::unit_vector(&(disc_sample - self.center));
        let cos_theta = sample_direction.dot(&disc.normal);
                    
        // below the hemisphere, integrand is 0
        (if cos_theta < 0.0 { 0.0 } else {
            self.sphere_disc_subtended_angle(disc)
        }, sample_direction)
    }

    pub fn sphere_disc_subtended_angle(&self, disc: &Disc) -> f64 {
        let c_to_disc_c = disc.center - self.center;
        let cos_angle = vector::unit_vector(&c_to_disc_c).dot(&disc.normal).abs();
        let d = c_to_disc_c.length();
        (disc.radius * disc.radius * std::f64::consts::PI) / (d * d) * cos_angle
    }
}
