use rand;
use rand::Rng;
use rand::ThreadRng;

use geometry::vector::Vec3;

//////////////////////////////////////////////////////////////////////////////

pub fn random_vec() -> Vec3 {
    let mut rng = rand::thread_rng();
    return Vec3::new(rng.gen::<f64>() * 2.0 - 1.0,
                     rng.gen::<f64>() * 2.0 - 1.0,
                     rng.gen::<f64>() * 2.0 - 1.0);
}

pub fn within_eps(v1: &Vec3, v2: &Vec3) -> bool
{
    (*v1 - *v2).length() < 1e-8
}

pub fn within_eps_f(v1: f64, v2: f64) -> bool
{
    (v1 - v2).abs() < 1e-8
}
