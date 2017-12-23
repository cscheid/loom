mod vector;
use vector::Vec3;

fn write_ppm() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new((i as f64) / (nx as f64),
                                (j as f64) / (ny as f64),
                                0.2);
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}

fn main() {
    write_ppm();
}
