use vector::Vec3;

use rand;
use rand::Rng;

//////////////////////////////////////////////////////////////////////////////

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = rand::thread_rng();
    
    while p.dot(&p) >= 1.0 {
        p = Vec3::new(rng.gen::<f64>() * 2.0 - 1.0,
                      rng.gen::<f64>() * 2.0 - 1.0,
                      rng.gen::<f64>() * 2.0 - 1.0);
    }

    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 0.0);
    let mut rng = rand::thread_rng();

    while p.dot(&p) >= 1.0 {
        p = Vec3::new(rng.gen::<f64>() * 2.0 - 1.0,
                      rng.gen::<f64>() * 2.0 - 1.0,
                      0.0);
    }

    p
}

