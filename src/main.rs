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
mod metal;
mod mixture;
mod dielectric;

use vector::Vec3;
use ray::Ray;
use rand::Rng;
use camera::Camera;
use lambertian::Lambertian;
use metal::Metal;
use mixture::Mixture;
use dielectric::Dielectric;

use sphere::*;
use hitable::*;
use hitable_list::*;

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
    let nx = 400;
    let ny = 200;
    let ns = 400;
    
    println!("P3\n{} {}\n255", nx, ny);

    let mut obj_list = Vec::<Box<Hitable>>::new();
    obj_list.push(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(&Vec3::new(0.8, 0.8, 0.0)))));
    for x in 1..4 {
        for y in 1..4 {
            for z in 1..4 {
                let xf = (x as f64) / 4.0;
                let yf = (y as f64) / 4.0;
                let zf = (z as f64) / 4.0;
                
                let material = if (x + y + z) % 2 == 0 {
                    Mixture::new(Metal::new(&Vec3::new(1.0, 1.0, 1.0)),
                                 Lambertian::new(&Vec3::new(xf, yf, zf)),
                                 0.8)
                } else {
                    Dielectric::new(1.5)
                };
                obj_list.push(Box::new(Sphere::new(Vec3::new(xf-(1.0/2.0),
                                                             yf-0.5,
                                                             -1.5+zf), 0.1,
                                                   material)));
            }
        }
    }
    let world = HitableList::new(obj_list);// vec![
        // Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(&Vec3::new(0.3, 0.3, 0.3)))),
        // Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Metal::new(&Vec3::new(0.8, 0.6, 0.2)))),
        // Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Metal::new(&Vec3::new(0.8, 0.8, 0.8))))
        //     ]);
    let camera = Camera::new(60.0, (nx as f64)/(ny as f64));
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
