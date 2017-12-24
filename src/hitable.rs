use vector::Vec3;
use ray::Ray;
use material::Material;
use std::rc::Rc;
use aabb::AABB;

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<Material>>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: None
        }
    }

    pub fn set(&mut self, other: &HitRecord) {
        self.t = other.t;
        self.p = other.p;
        self.normal = other.normal;
        self.material = other.material.as_ref().cloned();
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Option<AABB>;
}
