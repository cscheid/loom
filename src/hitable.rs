use vector::Vec3;
use ray::Ray;
use material::Material;
use aabb::AABB;

pub struct HitRecord<'a> {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'a Material
}

impl<'a> HitRecord<'a> {
    pub fn hit(t: f64, p: Vec3, normal: Vec3, material: &'a Material) -> HitRecord<'a> {
        HitRecord {
            t: t,
            p: p,
            normal: normal,
            material: material
        }
    }
}

pub trait Hitable : Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
    fn bounding_box(&self) -> Option<AABB>;
}
