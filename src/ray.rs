use vector::Vec3;
use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    A: Vec3,
    B: Vec3
}

impl Ray {
    #[inline]
    pub fn new(a: &Vec3, b: &Vec3) -> Ray {
        Ray { A: *a, B: *b }
    }

    #[inline]
    pub fn origin(&self) -> Vec3 { self.A }

    #[inline]
    pub fn direction(&self) -> Vec3 { self.B }

    #[inline]
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.A + t * self.B
    }
    
}

#[test]
fn it_doesnt_smoke()
{
    let r = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));
    println!("{:?}", r);
    println!("halfway point is: {:?}", r.point_at_parameter(0.5));
}
