use vector::Vec3;
use vector;
use ray::Ray;
use disc::Disc;

pub struct SphereGeom {
    pub center: Vec3,
    pub radius: f64
}

impl SphereGeom {
    pub fn new(center: Vec3, radius: f64) -> SphereGeom {
        SphereGeom { center: center, radius: radius }
    }

    pub fn intersect_ray(&self, r: &Ray) -> Option<(Vec3, Vec3)> {
        let oc = r.origin() - self.center;
        let a = vector::dot(&r.direction(), &r.direction());
        let b = vector::dot(&oc, &r.direction());
        let c = vector::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp1 = (-b - discriminant.sqrt()) / a;
            let temp2 = (-b + discriminant.sqrt()) / a;
            let p1 = r.point_at_parameter(temp1);
            let p2 = r.point_at_parameter(temp2);
            Some((p1, p2))
        } else {
            None
        }
    }
}
