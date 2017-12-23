extern crate rand;

mod vector;
mod ray;
mod hitable;
mod sphere;
mod hitable_list;
mod camera;
mod sampling;
mod material;
mod lambertian;

use vector::Vec3;
use ray::Ray;
use rand::Rng;
use camera::Camera;
use lambertian::Lambertian;

use sphere::*;
use hitable::*;
use hitable_list::*;
use sampling::*;

fn color(ray: &Ray, world: &Hitable, depth: i32) -> Vec3
{
    let mut r = HitRecord::new();

    if world.hit(ray, 0.001, 1e20, &mut r) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();
        if depth < 50 && r.material.as_ref().expect("right here, yes").scatter(ray, &r, &mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, depth+1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
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

    let s1 = Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(&Vec3::new(0.5, 0.5, 0.5))));
    let s2 = Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(&Vec3::new(0.8, 0.5, 0.1))));
    
    let world = HitableList::new(vec![s1, s2]);
    let camera = Camera::new();
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world, 0);
            }
            col = col / (ns as f64);
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
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
