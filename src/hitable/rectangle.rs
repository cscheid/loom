use geometry::aabb::AABB;
use geometry::aabb;
use geometry::ray::Ray;
use geometry::vector::Vec3;
use geometry::vector;
use hitable::*;
use material::*;
use std::rc::Rc;
    
// for tests
// use tests::*;
// use lambertian::Lambertian;

//////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Rectangle {
    pub bottom_left: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub material: Rc<Material>,
    pub normal: Vec3,
    pub bounding_box: Option<AABB>
}

impl Rectangle {
    pub fn new(bottom_left: Vec3, right: Vec3, up: Vec3,
               material: Rc<Material>) -> Rectangle {
        let p1 = bottom_left + right;
        let p2 = bottom_left + up;
        let p3 = p2 + right;
        
        // it's ok to be inefficient on construction
        let bb = Some(aabb::surrounding_box(
            &aabb::surrounding_box(&AABB::new(p1, p1),
                                   &AABB::new(p2, p2)),
            &aabb::surrounding_box(&AABB::new(p3, p3),
                                   &AABB::new(bottom_left, bottom_left))));
        
        Rectangle {
            bottom_left: bottom_left,
            right: right,
            up: up,
            material: material,
            normal: vector::unit_vector(&vector::cross(&right, &up)),
            bounding_box: bb
        }
    }
}

impl Hitable for Rectangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64,
           rec: &mut HitRecord) -> bool {
        // the intersection is given by the solution of
        // b: bottom_left
        // r: right
        // u: up
        // o: origin
        // d: direction
        //
        // b + alpha r + beta u = o + gamma d
        //  or
        // alpha r + beta u - gamma d = o - b
        //
        // this is a 3x3 linear system, which we'll solve by cramer's
        // rule (yeah, yuck) the ray intersects the rectangle at o +
        // gamma d iff 0<=alpha<=1, 0<=beta<=1

        let r = &self.right;
        let u = &self.up;
        let b = &self.bottom_left;
        let o = &ray.origin();
        let d = &ray.direction();

        let ob = *o - *b;

        let delta =
            r[0] * u[1] * d[2] + r[1] * u[2] * d[0] + r[2] * u[0] * d[1]
          - r[0] * u[2] * d[1] - r[1] * u[0] * d[2] - r[2] * u[1] * d[0];
        let delta_alpha =
            ob[0] * u[1] * d[2] + ob[1] * u[2] * d[0] + ob[2] * u[0] * d[1]
          - ob[0] * u[2] * d[1] - ob[1] * u[0] * d[2] - ob[2] * u[1] * d[0];
        let delta_beta =
            r[0] * ob[1] * d[2] + r[1] * ob[2] * d[0] + r[2] * ob[0] * d[1]
          - r[0] * ob[2] * d[1] - r[1] * ob[0] * d[2] - r[2] * ob[1] * d[0];
        let delta_gamma =
            r[0] * u[1] * ob[2] + r[1] * u[2] * ob[0] + r[2] * u[0] * ob[1]
          - r[0] * u[2] * ob[1] - r[1] * u[0] * ob[2] - r[2] * u[1] * ob[0];

        let alpha =   delta_alpha / delta;
        let beta  =   delta_beta  / delta;
        let gamma = - delta_gamma / delta;

        // eprintln!("Alpha: {}", alpha);
        // eprintln!("Beta: {}", beta);
        // eprintln!("Gamma: {}", gamma);

        if alpha <= 0.0 || alpha >= 1.0 || beta <= 0.0 || beta >= 1.0 ||
            gamma >= t_max || gamma <= t_min {
            false
        } else {
            rec.t = gamma;
            rec.p = ray.point_at_parameter(rec.t);
            rec.normal = self.normal;
            rec.material = Some(Rc::clone(&self.material));
            true
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.bounding_box
    }
}

#[test]
fn basic_tests()
{
    // let r1 = Rectangle::new(Vec3::new(0.0, 0.0, 0.0),
    //                         Vec3::new(1.0, 0.0, 0.0),
    //                         Vec3::new(0.0, 1.0, 0.0),
    //                         Lambertian::new(&Vec3::new(1.0, 1.0, 1.0)));

    // let r1 = Rectangle::new(Vec3::new( 0.25, 0.5, -0.25),
    //                         Vec3::new(-0.5,  0.0,  0.0),
    //                         Vec3::new( 0.0,  0.0,  0.5),
    //                         Lambertian::new(&Vec3::new(1.0, 1.0, 1.0)));

    // eprintln!("normal: {:?}", r1.normal);
    // let mut hr  = HitRecord::new();
    // let mut ray = Ray::new(Vec3::new(0.0,  2.0, 0.0),
    //                        Vec3::new(0.0, -1.0, 0.0));
    // assert!(r1.hit(&ray,
    //                0.0001,
    //                1e20,
    //                &mut hr));

    // // assert!(within_eps(&r1.normal, &Vec3::new(0.0, 0.0, 1.0)));

    // let mut a = Vec3::new(1.0, 1.0, 1.0);
    // let mut s = Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0));

    // for i in 0..10 {
    //     r1.material.scatter(&ray, &hr, &mut a, &mut s);
    //     eprintln!("Scatter: {:?}", s);
    // }
}

