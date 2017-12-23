mod vector;
mod ray;
mod hitable;
mod sphere;

use vector::Vec3;
use ray::Ray;
use hitable::HitRecord;
use sphere::Sphere;
use hitable::Hitable;

fn color(ray: &Ray) -> Vec3
{
    let mut r = HitRecord::new();
    let s = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    if s.hit(ray, 0.0, 1e20, &mut r) {
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
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);
            let r = Ray::new(origin,
                             lower_left_corner + u*horizontal + v*vertical);
            let col = color(&r);
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
