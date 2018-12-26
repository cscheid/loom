use vector::Vec3;
use vector;
use plane::Plane;
use ray::Ray;
use sphere_geometry::SphereGeom;
use sampling;
use tests::*;

use std::f64;

//////////////////////////////////////////////////////////////////////////////
// testing imports
#[allow(unused_imports)]
use sampling::*;
#[allow(unused_imports)]
use random::*;

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

    pub fn tangent_space(&self) -> (Vec3, Vec3) {
        // let u_vec = loop {
        //     let b = sampling::random_3d_direction();
        //     if b.dot(&self.normal).abs() < 0.05 {
        //         continue;
        //     }
        //     break vector::unit_vector(
        //         &vector::cross(&self.normal, &b));
        // };
        
        let mut t = vector::cross(&self.normal, &Vec3::new(1.0, 0.0, 0.0));
        if t.length() < 0.1 {
            t = vector::cross(&self.normal, &Vec3::new(0.0, 1.0, 0.0));
            if t.length() < 0.1 {
                t = vector::cross(&self.normal, &Vec3::new(0.0, 0.0, 1.0));
            }
            if t.length() < 0.1 {
                println!("{:?}", self.normal);
                panic!("WTFFFFFF");
            }
        }
        let u_vec = vector::unit_vector(&t);
            
        // let mut u_vec = vector::unit_vector(
        //     &vector::cross(&self.normal, &Vec3::new(1.0, 0.0, 0.0)));
        // if u_vec.length() == 0.0 {
        //     u_vec = vector::unit_vector(
        //         &vector::cross(&self.normal, &Vec3::new(0.0, 1.0, 0.0)));
        // }
        
        (u_vec, vector::unit_vector(
            &vector::cross(&self.normal, &u_vec)))
    }

    pub fn random(&self) -> Vec3 {
        let v = sampling::random_in_unit_disk();
        self.sample(v.x(), v.y())
    }
    
    pub fn sample(&self, u: f64, v: f64) -> Vec3 {
        let n = self.tangent_space();
        let u_vec = n.0;
        let v_vec = n.1;
        self.center + u_vec * (u * self.radius) + v_vec * (v * self.radius)
    }
    
    pub fn sphere(&self) -> SphereGeom {
        SphereGeom::new(self.center, self.radius)
    }

    pub fn new(center: Vec3, normal: Vec3, radius: f64) -> Disc {
        assert!(within_eps_f(normal.length(), 1.0));
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
        let cos_theta = sample_direction.dot(&self.normal);
                    
        // below the hemisphere, integrand is 0
        // (self.sphere_disc_subtended_angle(disc), sample_direction)
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

//////////////////////////////////////////////////////////////////////////////

#[test]
fn subtended_angle_estimation_works() {
    let hemi = Disc::new(Vec3::new(0.0, 0.0, 0.0),
                         Vec3::new(0.0, 1.0, 0.0),
                         1.0);

    let other = Disc::new(Vec3::new(-1.0, 0.0, 0.0),
                          Vec3::new( 1.0, 0.0, 0.0),
                          0.01);
    let other2 = Disc::new(Vec3::new(-0.707106, 0.707106, 0.0),
                           vector::unit_vector(&Vec3::new(0.707106, -0.707106, 0.0)),
                           0.01);
    let other3 = Disc::new(Vec3::new(-0.707106, 0.707106, 0.0),
                           Vec3::new(1.0, 0.0, 0.0),
                           0.01);

    let n = 100000;
    let mut itor1 = (0..n).map(|_| hemi.hemi_disc_subtended_angle(&other).0);
    let mut itor2 = (0..n).map(|_| hemi.hemi_disc_subtended_angle(&other2).0);
    let mut itor3 = (0..n).map(|_| hemi.hemi_disc_subtended_angle(&other3).0);
    let e1 = avstdev(&mut itor1).0;
    let e2 = avstdev(&mut itor2).0;
    let e3 = avstdev(&mut itor3).0;
    println!("Disc's subtended angle");
    println!("  Average: {}", e1);
    println!("  Half of disc area: {}", 0.01 * 0.01 * std::f64::consts::PI / 2.0);
    println!("  Average: {}", e2);
    println!("  disc area: {}", 0.01 * 0.01 * std::f64::consts::PI);
    println!("  Average: {}", e3);
    println!("  disc area: {}", 0.01 * 0.01 * std::f64::consts::PI * 0.707106);
}

#[test]
fn tangent_space_works()
{
    for _i in 0..100 {
        let hemi = Disc::new(sampling::random_in_unit_sphere(),
                             sampling::random_3d_direction(),
                             rand_double());
        let t = hemi.tangent_space();
        assert!(within_eps_f(t.0.length(), 1.0));
        assert!(within_eps_f(t.1.length(), 1.0));
        assert!(within_eps_f(t.0.dot(&hemi.normal), 0.0));
        assert!(within_eps_f(t.1.dot(&hemi.normal), 0.0));
        assert!(within_eps_f(t.0.dot(&t.1), 0.0));
    }
    for n in vec![Vec3::new(1.0, 0.0, 0.0),
                  Vec3::new(0.0, 1.0, 0.0),
                  Vec3::new(0.0, 0.0, 1.0)] {
        let hemi = Disc::new(sampling::random_in_unit_sphere(),
                             n,
                             rand_double());
        let t = hemi.tangent_space();
        assert!(within_eps_f(t.0.length(), 1.0));
        assert!(within_eps_f(t.1.length(), 1.0));
        assert!(within_eps_f(t.0.dot(&hemi.normal), 0.0));
        assert!(within_eps_f(t.1.dot(&hemi.normal), 0.0));
        assert!(within_eps_f(t.0.dot(&t.1), 0.0));
    }

    let hemi = Disc::new(
        Vec3::new(0.0, 0.0, 0.0),
        vector::unit_vector(&Vec3::new(1.0, 0.0, 1.0)),
        1.0);

    let t = hemi.tangent_space();
    println!("Tangent space of {:?}", hemi.normal);
    println!("{:?} {:?}", t.0, t.1);
}

#[test]
fn sample_generation_works()
{
    for _i in 0..100 {
        let hemi = Disc::new(sampling::random_in_unit_sphere(),
                             sampling::random_3d_direction(),
                             rand_double());
        for _j in 0..100 {
            let s = hemi.random();
            let d = (s - hemi.center).length();
            if d > hemi.radius {
                println!("FAILUREEE!!!!!");
                let t = hemi.tangent_space();
                println!("{:?}", t);
                println!("{:?} {:?} {:?}", hemi, s, d);
            }
            assert!(d <= hemi.radius);
        }

        let mut itor1 = (0..10000).map(|_| hemi.random().x());
        let mut itor2 = (0..10000).map(|_| hemi.random().y());
        let mut itor3 = (0..10000).map(|_| hemi.random().z());

        let t1 = t_stat(&mut itor1, hemi.center.x());
        let t2 = t_stat(&mut itor2, hemi.center.y());
        let t3 = t_stat(&mut itor3, hemi.center.z());
        
        if !(t1 > -5.0 && t1 < 5.0 &&
             t2 > -5.0 && t2 < 5.0 &&
             t3 > -5.0 && t3 < 5.0) {
            assert!(t1 > -5.0 && t1 < 5.0 &&
                    t2 > -5.0 && t2 < 5.0 &&
                    t3 > -5.0 && t3 < 5.0);
        }
    }
}

#[test]
fn sample_gen_2()
{
    let tc = Disc::new(Vec3::new(0.9645107046439914, 0.042040731802689635, 0.04997783330771366),
                       Vec3::new(0.20481474183922158, 0.8789346307808042, -0.4307259411035564),
                       0.36703039022477624);
    let mut itor1 = (0..10000).map(|_| tc.random().x());
    let mut itor2 = (0..10000).map(|_| tc.random().y());
    println!("sample_gen_2 asl;dkfjas;ldfj");
    println!("{:?}", avstdev(&mut itor1));
    println!("{:?}", avstdev(&mut itor2));
}
