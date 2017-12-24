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
use bvh::BVH;
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

use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io;
use std::rc::Rc;
use std::time::SystemTime;

//////////////////////////////////////////////////////////////////////////////

fn scene() -> HitableList
{
    let mut obj_list = Vec::<Box<Hitable>>::new();
    obj_list.push(Box::new(
        Sphere::new(Vec3::new(0.0, -100.36, -1.0), 100.0,
                    Lambertian::new(&Vec3::new(0.9, 0.9, 0.9)))));
    for x in 1..8 {
        for y in 1..8 {
            for z in 1..8 {
                let xf = (x as f64) / 8.0;
                let yf = (y as f64) / 8.0;
                let zf = (z as f64) / 8.0;
                let material = if (x + y + z) % 2 == 1 {
                    Mixture::new(Metal::new(&Vec3::new(1.0, 1.0, 1.0)),
                                 Lambertian::new(&Vec3::new(xf, yf, zf)),
                                 0.8)
                } else {
                    Dielectric::new(1.5)
                };
                obj_list.push(Box::new(Sphere::new(Vec3::new(xf-(1.0/2.0),
                                                             yf-0.5,
                                                             -1.5+zf), 0.05,
                                                   Rc::clone(&material))));
                // make glass spheres hollow
                if (x + y + z) % 2 == 0 {
                    obj_list.push(Box::new(Sphere::new(Vec3::new(xf-(1.0/2.0),
                                                                 yf-0.5,
                                                                 -1.5+zf), -0.045,
                                                       Rc::clone(&material))));
                }
            }
        }
    }
    HitableList::new(obj_list)
}

// fn scene() -> HitableList
// {
//     let mut obj_list = Vec::<Box<Hitable>>::new();
//     for i in 0..4 {
//         for j in 0..4 {
//             for k in 0..4 {
//                 obj_list.push(Box::new(
//                     Sphere::new(Vec3::new(i as f64, j as f64, k as f64), 0.5,
//                                 Lambertian::new(&Vec3::new(0.9, 0.9, 0.9)))));
//             }
//         }
//     }
//     HitableList::new(obj_list)
// }

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

fn write_image_to_file(image: &Vec<Vec<Vec3>>, samples_so_far: usize, subsample: usize, file_prefix: &String)
{
    let mut f = BufWriter::new(File::create(format!("{}.ppm", file_prefix)).unwrap());
    let ny = image.len()/subsample;
    let nx = image[0].len()/subsample;
    let ns = samples_so_far as f64;
    f.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny)).unwrap();
    for super_j in (0..ny).rev() {
        for super_i in 0..nx {
            let mut super_pixel = Vec3::zero();
            let top   = cmp::min(image.len(),    (super_j+1)*subsample);
            let right = cmp::min(image[0].len(), (super_i+1)*subsample);
            let h = top   - super_j*subsample;
            let w = right - super_i*subsample;
            for j in (super_j*subsample..top).rev() {
                for i in super_i*subsample..right {
                    super_pixel = super_pixel + image[j][i];
                }
            }
            let mut out_col = super_pixel / (ns * (w as f64) * (h as f64));
            out_col = Vec3::new(out_col[0].sqrt(), out_col[1].sqrt(), out_col[2].sqrt());
            let ir = (255.99 * out_col[0]) as i32;
            let ig = (255.99 * out_col[1]) as i32;
            let ib = (255.99 * out_col[2]) as i32;
            f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
        }
    }
}

fn write_multiple_images_to_file(image: &Vec<Vec<Vec3>>, ns: usize, name: &String)
{
    write_image_to_file(&image, ns, 1, &format!("{}-1", name));
    write_image_to_file(&image, ns, 2, &format!("{}-2", name));
    write_image_to_file(&image, ns, 4, &format!("{}-4", name));
    write_image_to_file(&image, ns, 8, &format!("{}-8", name));
}

fn write_image(args: &Args)
{
    let nx           = args.w.unwrap_or(200);
    let ny           = args.h.unwrap_or(100);
    let ns           = args.s.unwrap_or(100);
    let fov          = args.f.unwrap_or(90.0);
    let aperture     = args.a.unwrap_or(0.0);
    let focus_dist   = args.d.unwrap_or(1.0);
    let interval     = args.i.unwrap_or(600);
    
    let default_name = "out".to_string();
    let output_name  = &args.o.as_ref().unwrap_or(&default_name);
    
    let mut world = scene();
    let bvh_world = BVH::build(world.get_list_mut());

    let background = overhead_light();
    
    let camera = Camera::new(
        &Vec3::new(-0.2, 0.5, 0.0),
        &Vec3::new(0.0, 0.0, -1.0),
        &Vec3::new(0.0, 1.0, 0.0),
        fov, (nx as f64)/(ny as f64),
        aperture, focus_dist
    );
    let mut rng = rand::thread_rng();

    let mut output_image = Vec::<Vec<Vec3>>::new();
    for j in 0..ny {
        output_image.push(vec![Vec3::zero(); nx]);
    }

    let mut last_write = SystemTime::now();
    let mut current_iteration = 1;
    
    for s in 1..ns+1 {
        for j in (0..ny).rev() {
            for i in 0..nx {
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                output_image[j][i] = output_image[j][i] + color(&r, &*bvh_world, &background, 0);
                // output_image[j][i] = output_image[j][i] + color(&r, &world, &background, 0);
            }
        }
        match last_write.elapsed() {
            Ok(elapsed) => {
                // write images every minute.
                if elapsed.as_secs() >= interval {
                    last_write = SystemTime::now();
                    let name = format!("{}-{:04}", output_name, s);
                    write_multiple_images_to_file(&output_image, s, &name);
                }
            },
            Err(e) => {
                panic!("time travel on now.elapsed()");
            }
        }
    }
    let name = format!("{}-final", output_name);
    write_multiple_images_to_file(&output_image, ns, &name);
}

//////////////////////////////////////////////////////////////////////////////

struct Args {
    pub w: Option<usize>,
    pub h: Option<usize>,
    pub s: Option<usize>,
    pub f: Option<f64>,
    pub a: Option<f64>,
    pub d: Option<f64>,
    pub o: Option<String>,
    pub i: Option<u64>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("i", "input", "set interval between saving intermediate files", "NAME");
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
        w: matches.opt_str("w").and_then(|x| x.parse::<usize>().ok()),
        h: matches.opt_str("h").and_then(|x| x.parse::<usize>().ok()),
        s: matches.opt_str("s").and_then(|x| x.parse::<usize>().ok()),
        f: matches.opt_str("f").and_then(|x| x.parse::<f64>().ok()),
        a: matches.opt_str("a").and_then(|x| x.parse::<f64>().ok()),
        d: matches.opt_str("d").and_then(|x| x.parse::<f64>().ok()),
        i: matches.opt_str("i").and_then(|x| x.parse::<u64>().ok()),
        o: matches.opt_str("o")
    }));
}
