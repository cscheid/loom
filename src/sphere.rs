use aabb::AABB;
use hitable::*;
use material::*;
use ray::Ray;
use vector::Vec3;
use vector;

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<Material>) -> Sphere {
        Sphere { center: center, radius: radius, material: material }
    }
}

impl Hitable for Sphere {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64) ->
        Option<HitRecord<'a>> {
        let oc = r.origin() - self.center;
        let a = vector::dot(&r.direction(), &r.direction());
        let b = vector::dot(&oc, &r.direction());
        let c = vector::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp1 = (-b - discriminant.sqrt()) / a;
            if temp1 < t_max && temp1 > t_min {
                let t = temp1;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::hit(t, p, normal, &*self.material))
            }
            let temp2 = (-b + discriminant.sqrt()) / a;
            if temp2 < t_max && temp2 > t_min {
                let t = temp2;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord::hit(t, p, normal, &*self.material))
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        let r = self.radius.abs(); // works when radius is negative
        Some(AABB::new(self.center - Vec3::new(r, r, r),
                       self.center + Vec3::new(r, r, r)))
    }
}
