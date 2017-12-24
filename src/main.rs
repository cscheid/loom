extern crate rand;
extern crate getopts;

mod aabb;
mod background;
mod bvh;
mod camera;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod mixture;
mod ray;
mod sampling;
mod sphere;
mod vector;

use background::*;
use camera::Camera;
use dielectric::Dielectric;
use lambertian::Lambertian;
use metal::Metal;
use mixture::Mixture;
use rand::Rng;
use ray::Ray;
use vector::Vec3;
use hitable::*;
use hitable_list::*;
use sphere::*;

use getopts::Options;

use std::rc::Rc;
use std::env;

//////////////////////////////////////////////////////////////////////////////

fn scene() -> HitableList
{
    let mut obj_list = Vec::<Box<Hitable>>::new();
    obj_list.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.36, -1.0), 100.0,
                    Lambertian::new(&Vec3::new(0.9, 0.9, 0.9)))));
    for x in 1..4 {
        for y in 1..4 {
            for z in 1..4 {
                let xf = (x as f64) / 4.0;
                let yf = (y as f64) / 4.0;
                let zf = (z as f64) / 4.0;
                
                let material = if (x + y + z) % 2 == 1 {
                    Mixture::new(Metal::new(&Vec3::new(1.0, 1.0, 1.0)),
                                 Lambertian::new(&Vec3::new(xf, yf, zf)),
                                 0.8)
                } else {
                    Dielectric::new(1.5)
                };

                obj_list.push(Box::new(Sphere::new(Vec3::new(xf-(1.0/2.0),
                                                             yf-0.5,
                                                             -1.5+zf), 0.1,
                                                   Rc::clone(&material))));
                // make glass spheres hollow
                if (x + y + z) % 2 == 0 {
                    obj_list.push(Box::new(Sphere::new(Vec3::new(xf-(1.0/2.0),
                                                                 yf-0.5,
                                                                 -1.5+zf), -0.09,
                                                       Rc::clone(&material))));
                }
            }
        }
    }
    HitableList::new(obj_list)
}

//////////////////////////////////////////////////////////////////////////////

// Rust doesn't let me use max() because of NaNs, etc. Huh.
#[inline]
fn fmax(v1: f64, v2: f64) -> f64
{
    if v1 > v2 { v1 } else { v2 }
}

fn color(ray: &Ray, world: &Hitable,
         background: &Background,
         depth: i32) -> Vec3 where
{
    let mut r = HitRecord::new();

    if world.hit(ray, 0.001, 1e20, &mut r) {
        let mut scattered = Ray::zero();
        let mut attenuation = Vec3::zero();
        if depth < 50 &&
            r.material
            .as_ref()
            .expect("right here, yes")
            .scatter(ray, &r, &mut attenuation, &mut scattered) {
            attenuation * color(&scattered, world, background, depth+1)
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = vector::unit_vector(&ray.direction());
        background.get_background(&unit_direction)
    }
}

fn write_image(args: &Args)
{
    let nx         = args.w.unwrap_or(200);
    let ny         = args.h.unwrap_or(100);
    let ns         = args.s.unwrap_or(100);
    let fov        = args.f.unwrap_or(90.0);
    let aperture   = args.a.unwrap_or(0.0);
    let focus_dist = args.d.unwrap_or(1.0);
    
    println!("P3\n{} {}\n255", nx, ny);
    let world = scene();
    let background = sky(); // overhead_light();
    
    let camera = Camera::new(
        &Vec3::new(-0.2, 0.5, 0.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        fov, (nx as f64)/(ny as f64),
        aperture, focus_dist
    );
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world, &background, 0);
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

//////////////////////////////////////////////////////////////////////////////

struct Args {
    pub w: Option<i32>,
    pub h: Option<i32>,
    pub s: Option<i32>,
    pub f: Option<f64>,
    pub a: Option<f64>,
    pub d: Option<f64>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("w", "width", "set image width in pixels", "NAME");
    opts.optopt("h", "height", "set image height in pixels", "NAME");
    opts.optopt("s", "samples", "set number of samples per pixel", "NAME");
    opts.optopt("f", "fov", "set field of view in degrees", "NAME");
    opts.optopt("a", "aperture", "set aperture diameter", "NAME");
    opts.optopt("d", "distance", "set focus distance", "NAME");
    opts.optflag("?", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    write_image(&(Args {
        w: matches.opt_str("w").and_then(|x| x.parse::<i32>().ok()),
        h: matches.opt_str("h").and_then(|x| x.parse::<i32>().ok()),
        s: matches.opt_str("s").and_then(|x| x.parse::<i32>().ok()),
        f: matches.opt_str("f").and_then(|x| x.parse::<f64>().ok()),
        a: matches.opt_str("a").and_then(|x| x.parse::<f64>().ok()),
        d: matches.opt_str("d").and_then(|x| x.parse::<f64>().ok())
    }));
}
