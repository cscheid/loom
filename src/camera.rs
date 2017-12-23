use vector::Vec3;
use ray::Ray;
use std::f64::consts::PI;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        
        Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0)
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin,
                 self.lower_left_corner +
                 u * self.horizontal +
                 v * self.vertical)
    }
}
