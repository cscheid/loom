extern crate rand;
extern crate getopts;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate bincode;
extern crate rayon;

mod aabb;
mod background;
mod bvh;
mod camera;
mod deserialize;
mod dielectric;
mod disc;
mod emitter;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod mixture;
mod phong;
mod plane;
mod random;
mod ray;
mod rectangle;
mod scene;
mod sampling;
mod sphere;
mod sphere_geometry;
mod triangle_mesh;
mod vector;
mod tests;

use aabb::AABB;
use background::*;
use bvh::BVH;
use camera::Camera;
use deserialize::*;
use disc::*;
use getopts::Options;
use hitable::*;
use hitable_list::*;
use metal::Metal;
use rand::Rng;
use random::*;
use ray::Ray;
use scene::Scene;
use vector::Vec3;
    
use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::thread;
use std::time::SystemTime;

use rayon::prelude::*;
use serde_json::*;

//////////////////////////////////////////////////////////////////////////////

fn color(ray: &Ray, world: &Hitable,
         background: &Background,
         lights: &Vec<AABB>) -> Vec3 where
{
    let mut current_ray = *ray;
    let mut current_attenuation = Vec3::new(1.0, 1.0, 1.0);
        
    for depth in 0..50 {
        if current_attenuation.length() < 1e-8 {
            return Vec3::new(0.0, 0.0, 0.0)
        }
        
        match world.hit(&current_ray, 0.00001, 1e20) {
            None => {
                let unit_direction = vector::unit_vector(&current_ray.direction());
                return background.get_background(&unit_direction) * current_attenuation;
            },
            Some(r) => {
                if !r.material.wants_importance_sampling() || lights.len() == 0 {
                    match r.material.scatter(&current_ray, &r) {
                        material::Scatter::Bounce(next_attenuation, scattered) => {
                            current_attenuation = current_attenuation * next_attenuation;
                            current_ray = scattered;
                        },
                        material::Scatter::Emit(emission) => {
                            // println!("Hit light!");
                            return emission * current_attenuation;
                        },
                        material::Scatter::Absorb => {
                            return Vec3::new(0.0, 0.0, 0.0)
                        }
                    }
                    continue;
                }
                
                // 1-sample MC from importance-sampling distribution:
                let u = 0.9; // this should be a parameter carefully chosen
                let this_hemi = Disc::new(r.p, r.normal, 1.0);
                
                let next_values = if rand_double() < u { // sample from lights
                    // println!("Sampling from light.");
                    // choose a disc
                    let chosen_light = &lights[rand_range(0, lights.len())];
                    let chosen_disc = chosen_light.project_to_disc_on_sphere(&r.p);
                    // println!("This hemi: {:?}", this_hemi);
                    // println!("Sampling from disc: {:?}", &chosen_disc);

                    // sample from that disc
                    let gx_sample        = this_hemi.hemi_disc_subtended_angle(&chosen_disc);
                    let gx               = gx_sample.0;
                    let sample_direction = gx_sample.1;

                    let next_ray = Ray::new(r.p, sample_direction);
                    // println!("Disc sample: {:?}", &next_ray);
                    let fx = r.material.bsdf(&next_ray, &r.normal);
                    // println!("gx: {:?}", &gx_v);
                    // println!("fx: {:?}", &fx);
                    let next_attenuation = if fx.abs() < 1e-8 {
                        Vec3::new(0.0, 0.0, 0.0)
                    } else {
                        r.material.albedo(&next_ray, &r.normal) * (fx / ((1.0 - u) * gx + u * fx))
                    };
                    // println!("Next attenuation from lights sample: {:?}", &next_attenuation);

                    (next_attenuation, next_ray)
                } else {
                    match r.material.scatter(&current_ray, &r) {
                        material::Scatter::Bounce(attenuation, scattered) => {
                            // this is very very inefficient with many lights.
                            let gx : f64 = lights.iter()
                                .map(|l| l.project_to_disc_on_sphere(&r.p))
                                .filter(|d| d.intersect_ray(&scattered).is_some())
                                .map(|d| this_hemi.hemi_disc_subtended_angle(&d).0)
                                .sum();
                            let fx = r.material.bsdf(&current_ray, &r.normal);

                            let next_attenuation = if fx.abs() < 1e-8 {
                                Vec3::new(0.0, 0.0, 0.0)
                            } else {
                                attenuation * (fx / ((1.0 - u) * gx + u * fx))
                            };
                            // println!("Next attenuation from bsdf sample: {:?}", &next_attenuation);

                            (next_attenuation, scattered)
                        },
                        material::Scatter::Emit(emission) => {
                            return emission * current_attenuation;
                        },
                        material::Scatter::Absorb => {
                            return Vec3::new(0.0, 0.0, 0.0)
                        }
                    }
                };
                current_attenuation = current_attenuation * next_values.0;
                current_ray = next_values.1;
            }
        }
    }
    current_attenuation
}

//////////////////////////////////////////////////////////////////////////////
// my own bastardized version of a float file format, horrendously inefficient

fn write_image_to_file(image: &Vec<Vec<Vec3>>, samples_so_far: usize, subsample: usize, file_prefix: &String)
{
    println!("Writing output to {}",
             format!("{}.linear_rgb", file_prefix));
    let mut f = BufWriter::new(File::create(format!("{}.linear_rgb", file_prefix)).unwrap());
    let ny = image.len()/subsample;
    let nx = image[0].len()/subsample;
    let ns = samples_so_far as f64;
    f.write_fmt(format_args!("{} {}\n", nx, ny)).unwrap();

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
            f.write_fmt(format_args!("{} {} {}\n", out_col[0], out_col[1], out_col[2])).unwrap();
        }
    }
}

fn update_all_pixels(output_image: &mut Vec<Vec<Vec3>>,
                     camera: &Camera,
                     bvh_world: &Hitable,
                     background: &Background,
                     lights: &Vec<AABB>,
                     nx: usize,
                     ny: usize,
                     rng: &mut rand::ThreadRng) {
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
            let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);
            let r = camera.get_ray(u, v);
            output_image[j][i] = output_image[j][i] + color(&r, bvh_world, background, lights);
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct ImageSummaries {
    w: usize,
    h: usize,
    s: usize,
    data: Vec<Vec<Vec3>>
}

fn combine_summaries(summary1: &ImageSummaries,
                     summary2: &ImageSummaries) -> ImageSummaries {
    if summary1.w != summary2.w {
        panic!(format!("Need same widths ({} vs {})!",
                       summary1.w, summary2.w));
    }
    if summary1.h != summary2.h {
        panic!(format!("Need same heights ({} vs {})!",
                       summary1.h, summary2.h));
    }
    if summary1.data.len() != summary2.data.len() {
        panic!(format!("Inconsistent data lengths ({} {}) - upstream bug?",
                       summary1.data.len(), summary2.data.len()));
    }
    let mut result = Vec::new();
    for i in 0..summary1.data.len() {
        let l1 = summary1.data[i].len();
        let l2 = summary2.data[i].len();
        if l1 != l2 {
            panic!(format!(
                "Inconsistent row lengths (row {}: {} {}) - upstream bug?",
                i, l1, l2));
        }
        let row1 = summary1.data[i].iter();
        let row2 = summary2.data[i].iter();
        result.push(row1.zip(row2).map(|(v1, v2)| *v1 + *v2).collect())
    }
    ImageSummaries {
        w: summary1.w,
        h: summary1.h,
        s: summary1.s + summary2.s,
        data: result
    }
}

fn write_image(args: &Args)
{
    let default_output_name = "out".to_string();
    let output_name         = &args.o.as_ref().unwrap_or(&default_output_name);

    let default_input_name  = "/dev/stdin".to_string();
    let input_name          = &args.i.as_ref().unwrap_or(&default_input_name);

    let br = BufReader::new(File::open(input_name).unwrap());
    let json_value = serde_json::from_reader(br).unwrap();

    let scene          = deserialize_scene(&json_value).unwrap();
    let background     = scene.background;
    let camera         = scene.camera;
    let lights: Vec<_> = scene.object_list
        .iter()
        .map(|h| h.importance_distribution())
        .filter(|h| h.is_some())
        .map(|h| h.unwrap())
        .collect();
    println!("Found {} lights", lights.len());
    println!("Light 1: {:?}", lights[0]);
    let v1 = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(-2.0, 2.0, 0.0);
    let v3 = Vec3::new(0.0, -2.0, 0.0);
    println!(" projection onto {:?}: {:?}", v1, lights[0].project_to_disc_on_sphere(&v1));
    println!(" projection onto {:?}: {:?}", v2, lights[0].project_to_disc_on_sphere(&v2));
    println!(" projection onto {:?}: {:?}", v3, lights[0].project_to_disc_on_sphere(&v3));
    let bvh_world      = BVH::build(scene.object_list);
    let ny             = args.h.unwrap_or(200);
    let nx             = args.w.unwrap_or_else(|| ((ny as f64) * camera.params.aspect).round() as usize);
    let n_threads      = args.n.unwrap_or(1);
    let ns             = args.s.unwrap_or(100) / n_threads;
    let background_ref = &*background;
    let bvh_world_ref  = &*bvh_world;
    println!("With {} threads", n_threads);

    let output_summaries: Vec<_> = (0..n_threads).into_par_iter().map(|i| {
        let mut output_image = Vec::<Vec<Vec3>>::new();
        for _j in 0..ny {
            output_image.push(vec![Vec3::zero(); nx]);
        }
        let mut rng = rand::thread_rng();
        let mut last_write = SystemTime::now();
            
        for s in 1..ns+1 {
            update_all_pixels(&mut output_image,
                              &camera, bvh_world_ref, background_ref, &lights,
                              nx, ny, &mut rng);
            if i == 0 {
                eprint!("\r                          \r{} / {} done", s, ns);
            }
        }
        if i == 0 {
            eprintln!("\nFinished");
        }
            
        ImageSummaries {
            w: nx,
            h: ny,
            s: ns,
            data: output_image
        }
    }).collect();
    
    let mut summary = output_summaries[0].clone();
    for new_summary in output_summaries.iter().skip(1) {
        summary = combine_summaries(&summary, &new_summary);
    }

    println!("Using {} samples", summary.s);
    write_image_to_file(&summary.data, summary.s, 1, &output_name);
}

//////////////////////////////////////////////////////////////////////////////

struct Args {
    pub w: Option<usize>,
    pub h: Option<usize>,
    pub s: Option<usize>,
    pub n: Option<usize>,
    pub o: Option<String>,
    pub i: Option<String>,
    pub parallel: bool
}

fn main() {
    random::init_rng();
    
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();

    opts.optopt("w", "width", "set image width in pixels", "NAME");
    opts.optopt("h", "height", "set image height in pixels", "NAME");
    opts.optopt("s", "samples", "set number of samples per pixel", "NAME");
    opts.optopt("n", "nthreads", "number of threads, default 1", "NAME");
    opts.optopt("o", "output", "set output file name", "NAME");
    opts.optopt("i", "input", "set input file name", "NAME");
    opts.optflag("p", "parallel", "write out pixel statistics, suited for parallel processing");

    opts.optflag("?", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    write_image(&(Args {
        w: matches.opt_str("w").and_then(|x| x.parse::<usize>().ok()),
        h: matches.opt_str("h").and_then(|x| x.parse::<usize>().ok()),
        s: matches.opt_str("s").and_then(|x| x.parse::<usize>().ok()),
        n: matches.opt_str("n").and_then(|x| x.parse::<usize>().ok()),

        o: matches.opt_str("o"),
        i: matches.opt_str("i"),
        parallel: matches.opt_present("p")
    }));
}
