extern crate rand;

mod vector;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;

use vector::Vec3;
use ray::Ray;
use rand::Rng;
use camera::Camera;

use hitable::*;
use sphere::*;
use hitable::*;
use hitable_list::*;

fn color(ray: &Ray, world: &Hitable) -> Vec3
{
    let mut r = HitRecord::new();

    if world.hit(ray, 0.0, 1e20, &mut r) {
        return 0.5 * (r.normal + Vec3::new(1.0, 1.0, 1.0));
    } else {
        let unit_direction = vector::unit_vector(&ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        vector::lerp(&Vec3::new(1.0, 1.0, 1.0),
                     &Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn write_image() {
    let nx = 200;
    let ny = 100;
    let ns = 100;
    
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let s1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5));
    let s2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0));
    
    let world = HitableList::new(vec![s1, s2]);
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for s in 0..ns {
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world);
            }
            col = col / (ns as f64);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    write_image();
}
