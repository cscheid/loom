use vector::Vec3;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    a: Vec3,
    b: Vec3
}

impl Ray {
    #[inline]
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray { a: a, b: b }
    }

    #[inline]
    pub fn origin(&self) -> Vec3 { self.a }

    #[inline]
    pub fn direction(&self) -> Vec3 { self.b }

    #[inline]
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.a + t * self.b
    }
    
}

#[test]
fn it_doesnt_smoke()
{
    let r = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    println!("{:?}", r);
    println!("origin of r: {:?}", r.origin());
    println!("direction of r: {:?}", r.direction());
    println!("halfway point is: {:?}", r.point_at_parameter(0.5));
}
