use hitable::*;
use vector::Vec3;
use ray::Ray;
use vector;
use material::*;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Rc<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere { center: center, radius: radius, material: material }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = vector::dot(&r.direction(), &r.direction());
        let b = vector::dot(&oc, &r.direction());
        let c = vector::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp1 = (-b - discriminant.sqrt()) / a;
            if temp1 < t_max && temp1 > t_min {
                rec.t = temp1;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
                return true;
            }
            let temp2 = (-b + discriminant.sqrt()) / a;
            if temp2 < t_max && temp2 > t_min {
                rec.t = temp2;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = Some(Rc::clone(&self.material));
                return true;
            }
        }
        return false;
    }
}
