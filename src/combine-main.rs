extern crate rand;
extern crate getopts;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate bincode;

mod aabb;
mod background;
mod bvh;
mod camera;
mod deserialize;
mod dielectric;
mod hitable;
mod hitable_list;
mod lambertian;
mod material;
mod metal;
mod mixture;
mod random;
mod rectangle;
mod ray;
mod scene;
mod sampling;
mod sphere;
mod triangle_mesh;
mod vector;
mod tests;

use background::*;
use bvh::BVH;
use camera::Camera;
use deserialize::*;
use hitable_list::*;
use metal::Metal;
use rand::Rng;
use ray::Ray;
use scene::Scene;
use vector::Vec3;
use hitable::*;

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

fn load(filename: &String) -> ImageSummaries {
    let mut f = BufReader::new(File::open(filename).unwrap());
    bincode::deserialize_from(&mut f, bincode::Infinite).unwrap()
}

//////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();

    let out_name = &args[1];

    println!("Will attempt to load {}", &args[2]);
    let mut summary = load(&args[2]);
    println!("Loaded {} ok.", &args[2]);
    for arg in args.iter().skip(3) {
        let new_summary = load(arg);
        println!("Loaded {} ok.", &arg);
        summary = combine_summaries(&summary, &new_summary);
    }

    write_image_to_file(&summary.data,
                        summary.s,
                        1,
                        out_name);
}
