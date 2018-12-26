use material::*;
use vector::Vec3;
use vector;
use ray::Ray;
use hitable::*;
use random::*;

use std::fmt;
use std::fmt::Debug;
use std::f64;

// Notes on the Ward BRDF, (Walter 2005)
// https://pdfs.semanticscholar.org/330e/59117d7da6c794750730a15f9a178391b9fe.pdf

#[derive(Debug)]
pub struct Ward {
    albedo: Vec3,
    alpha: f64,
    rho_s: f64
}

impl Ward {
    pub fn new(albedo: &Vec3, alpha: f64, rho_s: f64) -> Box<Material> {
        Box::new(Ward {
            albedo: *albedo,
            alpha: alpha,
            rho_s: rho_s
        })
    }
}

// isotropic Ward for now
impl Material for Ward {
    fn is_emitter(&self) -> bool { false }

    fn wants_importance_sampling(&self) -> bool { true }
    
    fn albedo(&self, _ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> Vec3 {
        self.albedo * surface_normal.dot(&ray_out.direction())
    }

    fn bsdf(&self, ray_in: &Ray, ray_out: &Ray, n: &Vec3) -> f64 {

        let i = -vector::unit_vector(&ray_in.direction());
        let o = ray_out.direction();
        if o.dot(n) <= 1e-8 {
            return 0.0;
        }
        let h = &vector::lerp(&i, &o, 0.5); // unnormalized half-vector works, see Walter 2005

        let ts = vector::tangent_space(n);
        let x = ts.0;
        let y = ts.1;
        let a = self.alpha;

        // equation (4) in Walter 2005
        let t1 = self.rho_s / (4.0 * std::f64::consts::PI *
                               a * a * (i.dot(n) * o.dot(n)).sqrt());

        let t2 = - ((h.dot(&x) / a).powi(2) + (h.dot(&y) / a).powi(2)) /
            h.dot(&n).powi(2);
        if t1 < 0.0 {
            println!("{} {}", t1, t2);
        }
        t1 * t2.exp()
    }

    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Scatter
    {
        let u = rand_double();
        let theta_v = rand_double();
        let n = &hr.normal;
        let ts = vector::tangent_space(n);
        let i = -vector::unit_vector(&ray_in.direction());
        if i.dot(n) <= 1e-8 {
            return Scatter::Absorb;
        }

        let phi_h = 2.0 * std::f64::consts::PI * theta_v;
        let phi_h_sincos = phi_h.sin_cos();
        let a2 = self.alpha * self.alpha;
        let theta_h = (-u.ln() / (phi_h_sincos.0.powi(2) / a2 +
                                  phi_h_sincos.1.powi(2) / a2)).atan();

        // I'm sure this is a dumb way of doing this.
        let h = vector::rotate(&vector::rotate(&hr.normal, &ts.1, theta_h),
                               &hr.normal, phi_h);
        let o = 2.0 * (i.dot(&h) * h) - i;
        // println!("{} {} {}", i.dot(&n), h.dot(&n), o.dot(&n));

        // fixme, add the weighting term here from Equation (10) in Walter 2005
        let rho_s = self.rho_s;
        Scatter::Bounce(Vec3::new(rho_s, rho_s, rho_s),
                        Ray::new(hr.p, o))
    }

    fn debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt(f)
    }
}
