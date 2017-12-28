extern crate rand;
extern crate getopts;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate bincode;

mod aabb;
mod background;
mod camera;
mod deserialize;
mod dielectric;
mod hitable;
mod lambertian;
mod material;
mod metal;
mod mixture;
mod random;
mod ray;
mod scene;
mod sampling;
mod vector;
mod tests;

use background::*;
use camera::Camera;
use deserialize::*;
use hitable::*;
use hitable::bvh::BVH;
use hitable::hitable_list::*;
use metal::Metal;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use vector::Vec3;

use getopts::Options;

use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;
use std::rc::Rc;
use std::time::SystemTime;

use serde_json::*;

//////////////////////////////////////////////////////////////////////////////

fn color(ray: &Ray, world: &Hitable,
         background: &Rc<Background>,
         depth: i32) -> Vec3 where
{
    let mut r = HitRecord::new();

    if world.hit(ray, 0.00001, 1e20, &mut r) {
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ImageSummaries {
    w: usize,
    h: usize,
    s: usize,
    data: Vec<Vec<Vec3>>
}

fn write_sample_summaries_to_file(image: &Vec<Vec<Vec3>>, ns: usize, name: &String)
{
    let mut f = BufWriter::new(File::create(format!("{}.bincode", name)).unwrap());
    
    let summary = ImageSummaries {
        w: image[0].len(),
        h: image.len(),
        s: ns,
        data: image.to_owned()
    };

    bincode::serialize_into(&mut f, &summary, bincode::Infinite);
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
    let interval     = args.t.unwrap_or(600);
    
    let default_output_name = "out".to_string();
    let output_name         = &args.o.as_ref().unwrap_or(&default_output_name);

    let default_input_name  = "/dev/stdin".to_string();
    let input_name          = &args.i.as_ref().unwrap_or(&default_input_name);

    let br = BufReader::new(File::open(input_name).unwrap());
    let json_value = serde_json::from_reader(br).unwrap();

    let scene = deserialize_scene(&json_value).unwrap();

    let bvh_world = BVH::build(scene.object_list);

    let background = scene.background;

    let camera = scene.camera;

    let ny           = args.h.unwrap_or(200);
    let nx           = args.w.unwrap_or_else(|| ((ny as f64) * camera.params.aspect).round() as usize);
    let ns           = args.s.unwrap_or(100);

    let mut rng = rand::thread_rng();

    let mut output_image = Vec::<Vec<Vec3>>::new();
    for _j in 0..ny {
        output_image.push(vec![Vec3::zero(); nx]);
    }

    let mut wrote_anything = false;
    let mut last_write = SystemTime::now();
    
    for s in 1..ns+1 {
        for j in (0..ny).rev() {
            for i in 0..nx {
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
                let r = camera.get_ray(u, v);
                output_image[j][i] = output_image[j][i] + color(&r, &*bvh_world, &background, 0);
            }
        }
        match last_write.elapsed() {
            Ok(elapsed) => {
                if elapsed.as_secs() >= interval || !wrote_anything {
                    wrote_anything = true;
                    last_write = SystemTime::now();
                    if args.parallel {
                        write_sample_summaries_to_file(&output_image, s, &output_name);
                    } else {
                        let name = format!("{}-{:04}", output_name, s);
                        write_multiple_images_to_file(&output_image, s, &name);
                    }
                }
            },
            Err(_e) => {
                panic!("time travel on now.elapsed()");
            }
        }
    }
    if args.parallel {
        write_sample_summaries_to_file(&output_image, ns, &output_name);
    } else {
        let name = format!("{}-final", output_name);
        write_multiple_images_to_file(&output_image, ns, &name);
    }
}

//////////////////////////////////////////////////////////////////////////////

struct Args {
    pub w: Option<usize>,
    pub h: Option<usize>,
    pub s: Option<usize>,

    // pub f: Option<f64>,
    // pub a: Option<f64>,
    // pub d: Option<f64>,

    pub o: Option<String>,
    pub i: Option<String>,
    pub t: Option<u64>,
    pub parallel: bool
}

fn main() {
    random::init_rng();
    
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("i", "input", "set input file name", "NAME");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("t", "interval", "set interval between saving intermediate files", "NAME");

    opts.optopt("w", "width", "set image width in pixels", "NAME");
    opts.optopt("h", "height", "set image height in pixels", "NAME");
    opts.optopt("s", "samples", "set number of samples per pixel", "NAME");
    // opts.optopt("f", "fov", "set field of view in degrees", "NAME");
    // opts.optopt("a", "aperture", "set aperture diameter", "NAME");
    // opts.optopt("d", "distance", "set focus distance", "NAME");
    opts.optflag("?", "help", "print this help menu");
    opts.optflag("p", "parallel", "write out pixel statistics, suited for parallel processing");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    write_image(&(Args {
        s: matches.opt_str("s").and_then(|x| x.parse::<usize>().ok()),
        w: matches.opt_str("w").and_then(|x| x.parse::<usize>().ok()),
        h: matches.opt_str("h").and_then(|x| x.parse::<usize>().ok()),

        t: matches.opt_str("t").and_then(|x| x.parse::<u64>().ok()),
        i: matches.opt_str("i"),
        o: matches.opt_str("o"),
        parallel: matches.opt_present("p")
    }));
}
