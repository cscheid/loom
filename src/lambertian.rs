use material::*;
use vector::Vec3;
use vector;
use sampling;
use ray::Ray;
use hitable::*;

use std::fmt;
use std::fmt::Debug;
use std::f64;

// testing imports
#[allow(unused_imports)]
use random::*;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3
}

// two-sided lambertian
impl Material for Lambertian {
    fn wants_importance_sampling(&self) -> bool { true }
    fn albedo(&self, _ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> Vec3 {
        self.albedo * surface_normal.dot(&ray_out.direction())
    }
    
    fn bsdf(&self, _ray_in: &Ray, ray_out: &Ray, surface_normal: &Vec3) -> f64 {
        let x = vector::unit_vector(&ray_out.direction()).dot(surface_normal);
        if x <= 0.0 {
            0.0
        } else {
            // std::f64::consts::PI * x integrates to PI/2 over
            // all directions of the hemisphere
            // so x * 2 integrates to 1 over all directions
            2.0 * x
        }
    }

    // generate a sample of ray_out distributed according to the bsdf
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Scatter {
        let target;
        if rec.normal.dot(&ray_in.direction()) > 0.0 {
            target = rec.p + sampling::random_3d_direction() - rec.normal;
        } else {
            target = rec.p + sampling::random_3d_direction() + rec.normal;
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

//////////////////////////////////////////////////////////////////////////////

#[test]
fn bsdf_is_a_pdf() {
    let m = Lambertian::new(&Vec3::new(1.0, 1.0, 1.0));
    let n = 100000;
    let sufficient = (0..n)
        .map(|_| m.bsdf(
            &Ray::new(Vec3::new(0.0, 0.0, 0.0),
                      Vec3::new(0.0, 1.0, 0.0)),
            &Ray::new(Vec3::new(0.0, 0.0, 0.0),
                      sampling::random_3d_direction()),
            &Vec3::new(0.0, 1.0, 0.0)))
        .filter(|x| x > &1e-8)
        .fold((0.0, 0.0, 0.0), |acc, next| {
            (acc.0+1.0, acc.1+next, acc.2+next*next)
        });
    // quick and dirty one-sample t-test 
    let n   = sufficient.0;
    let ex  = sufficient.1/n;
    let exx = sufficient.2/n;
    let variance = exx - ex * ex;
    let t   = (ex - 1.0) / (variance.sqrt() / (n as f64).sqrt());
    println!("testing if bsdf is a pdf:");
    println!("  Average: {}", ex);
    println!("  Variance: {}", variance);
    assert!(t < 1.96 && t > -1.96);
}

#[test]
fn integral_cosine_over_the_hemisphere() {
    let n = 1000000;
    let sufficient = (0..n)
        .map(|_| sampling::random_3d_direction())
        .map(|v| v.dot(&Vec3::new(0.0, 1.0, 0.0)))
        .filter(|v| v > &1e-8)
        .fold((0.0, 0.0, 0.0), |acc, next| {
            (acc.0+1.0, acc.1+next, acc.2+next*next)
        });
    let n   = sufficient.0;
    let ex  = sufficient.1/n;
    let exx = sufficient.2/n;
    let variance = exx - ex * ex;

    // integral of cos(x) over hemisphere = pi
    // hemisphere area * ex = pi
    // 2 pi * ex = pi
    // ex = 1/2

    println!("integral of cosine(normal) over the hemisphere");
    println!("  Average: {}", ex);
    println!("  Variance: {}", variance);
    let t   = (ex - 0.5) / (variance.sqrt() / (n as f64).sqrt());
    assert!(t < 1.96 && t > -1.96);
}

#[test]
fn scatter_obeys_cosine_law() {
    let m = Lambertian::new(&Vec3::new(1.0, 1.0, 1.0));
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let hr = HitRecord::hit(0.0, Vec3::new(0.0, 0.0, 0.0),
                            normal, &*m);
    let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0),
                       Vec3::new(0.0, -1.0, 0.0));
    
    let n = 1000000;
    let sufficient = (0..n)
        .map(|_| m.scatter(&ray, &hr))
        .map(|s| match s {
            Scatter::Bounce(_, r) => Some(r),
            Scatter::Emit(_) => None,
            Scatter::Absorb => None
        })
        .filter(|s| s.is_some())
        .map(|s| s.unwrap())
        .map(|scatter_ray| {
            // these samples are generated, presumably, from a
            // distribution weighted by the cosine of the angle
            // between the hemisphere normal and the point.
            // if we divide by the measure, then,
            // we should get an expectation.

            let f = m.bsdf(&ray, &scatter_ray, &normal);
            1.0/f
        })
        .fold((0.0, 0.0, 0.0), |acc, next| {
            (acc.0+1.0, acc.1+next, acc.2+next*next)
        });

    // integral of 1 over the hemisphere == 2pi
    // hemisphere area * ex = 2pi
    // ex = 1
    
    // quick and dirty one-sample t-test 
    let n   = sufficient.0;
    let ex  = sufficient.1/n;
    let exx = sufficient.2/n;
    let variance : f64 = exx - ex * ex;
    let t   = (ex - 1.0) / (variance.sqrt() / (n as f64).sqrt());
    println!("Testing lambertian scatter");
    println!("  Average: {}", ex);
    println!("  Variance: {}", variance);
    assert!(t < 1.96 && t > -1.96);
}
