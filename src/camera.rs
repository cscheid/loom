use ray::Ray;
use sampling;
use std::f64::consts::PI;
use vector::*;
use vector::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,

    // making serialization easier
    params: CameraParams
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, vup: &Vec3,
               vfov: f64, aspect: f64,
               aperture: f64, focus_dist: f64) -> Camera {
        
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = unit_vector(&(*look_from - *look_at));
        let u = unit_vector(&cross(vup, &w));
        let v = cross(&w, &u);

        Camera {
            lower_left_corner: *look_from - half_width*focus_dist*u - half_height*focus_dist*v - focus_dist*w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: *look_from,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w,
            params: CameraParams {
                look_from: *look_from,
                look_at: *look_at,
                vup: *vup,
                vfov: vfov,
                aspect: aspect,
                aperture: aperture,
                focus_dist: focus_dist
            }
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * sampling::random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(self.origin + offset,
                 self.lower_left_corner +
                 s * self.horizontal +
                 t * self.vertical - self.origin - offset)
    }
}

//////////////////////////////////////////////////////////////////////////////
// serialization

#[derive(Clone, Copy, Debug)]
pub struct CameraParams {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub vup: Vec3,
    pub vfov: f64,
    pub aspect: f64,
    pub aperture: f64,
    pub focus_dist: f64
}

// impl FromParam for Camera {
//     type Param = CameraParams;

//     fn thaw(params: CameraParams) -> Camera {
//         Camera::new(&params.look_from,
//                     &params.look_at,
//                     &params.vup,
//                     params.vfov,
//                     params.aspect,
//                     params.aperture,
//                     params.focus_dist)
//     }

//     fn freeze(&self) -> CameraParams {
//         self.params
//     }
// }
