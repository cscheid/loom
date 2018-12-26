extern crate rand;
extern crate getopts;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate bincode;

mod vector;
mod tests;
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

//////////////////////////////////////////////////////////////////////////////

fn luminance_stats(image: &Vec<Vec<Vec3>>, samples_so_far: usize, subsample: usize) -> (f64, f64)
{
    // 3.1: log-average luminance
    let nx = image[0].len()/subsample;
    let ny = image.len()/subsample;
    let nx = image[0].len()/subsample;
    let ns = samples_so_far as f64;

    let mut luminance_sum = 0.0 as f64;
    let mut pixel_count = 0;
    let mut max_lum = 0.0 as f64;
        
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
            pixel_count += 1;
            let lum = vector::luminance(&out_col);
            max_lum = f64::max(max_lum, lum);
            let delta = 0.01;
            
            luminance_sum += (delta + lum).ln();
        }
    }
    
    ((luminance_sum / pixel_count as f64).exp(), max_lum)
}

// (parts of) reinhard tone mapping
fn tonemap_image(image: &Vec<Vec<Vec3>>, samples_so_far: usize, subsample: usize, file_prefix: &String)
{
    let stats = luminance_stats(image, samples_so_far, subsample);
    let log_average = stats.0;
    let max_luminance = stats.1;
    
    println!("Log average luminance: {}", log_average);

    let a = 0.18;
    
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
            let l_w = vector::luminance(&out_col);
            let l_white = max_luminance;

            let l = (a / log_average) * l_w;
            let l_d = l * (1.0 + (l / (l_white * l_white))) / (1.0 + l);

            let lum_scale = l_d / l_w;

            let ir = cmp::max(0, cmp::min((255.99 * out_col[0] * lum_scale) as i32, 255));
            let ig = cmp::max(0, cmp::min((255.99 * out_col[1] * lum_scale) as i32, 255));
            let ib = cmp::max(0, cmp::min((255.99 * out_col[2] * lum_scale) as i32, 255));
            f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
        }
    }

    
}

fn write_image_to_file(image: &Vec<Vec<Vec3>>, samples_so_far: usize, subsample: usize, file_prefix: &String)
{
    tonemap_image(image, samples_so_far, subsample, file_prefix);
    // let mut f = BufWriter::new(File::create(format!("{}.ppm", file_prefix)).unwrap());
    // let ny = image.len()/subsample;
    // let nx = image[0].len()/subsample;
    // let ns = samples_so_far as f64;
    // f.write_fmt(format_args!("P3\n{} {}\n255\n", nx, ny)).unwrap();
    // for super_j in (0..ny).rev() {
    //     for super_i in 0..nx {
    //         let mut super_pixel = Vec3::zero();
    //         let top   = cmp::min(image.len(),    (super_j+1)*subsample);
    //         let right = cmp::min(image[0].len(), (super_i+1)*subsample);
    //         let h = top   - super_j*subsample;
    //         let w = right - super_i*subsample;
    //         for j in (super_j*subsample..top).rev() {
    //             for i in super_i*subsample..right {
    //                 super_pixel = super_pixel + image[j][i];
    //             }
    //         }
    //         let mut out_col = super_pixel / (ns * (w as f64) * (h as f64));

    //         // very basic gamma=2 mapping
    //         out_col = Vec3::new(out_col[0].sqrt(), out_col[1].sqrt(), out_col[2].sqrt());
    //         let ir = cmp::min((255.99 * out_col[0]) as i32, 255);
    //         let ig = cmp::min((255.99 * out_col[1]) as i32, 255);
    //         let ib = cmp::min((255.99 * out_col[2]) as i32, 255);
    //         f.write_fmt(format_args!("{} {} {}\n", ir, ig, ib)).unwrap();
    //     }
    // }
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

fn load(filename: &String) -> ImageSummaries {
    let mut f = BufReader::new(File::open(filename).unwrap());
    bincode::deserialize_from(&mut f, bincode::Infinite).unwrap()
}

//////////////////////////////////////////////////////////////////////////////

fn main() {
    let args: Vec<String> = env::args().collect();

    let out_name = &args[1];

    let mut summary = load(&args[2]);
    eprintln!("Loaded {} ok.", &args[2]);
    for arg in args.iter().skip(3) {
        let new_summary = load(arg);
        eprintln!("Loaded {} ok.", &arg);
        summary = combine_summaries(&summary, &new_summary);
    }

    eprintln!("Combined sample count: {}", summary.s);

    write_image_to_file(&summary.data,
                        summary.s,
                        1,
                        out_name);
}
