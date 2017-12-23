use vector::Vec3;

use rand;
use rand::Rng;

//////////////////////////////////////////////////////////////////////////////

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = rand::thread_rng();
    
    while p.length() >= 1.0 {
        p = Vec3::new(rng.gen::<f64>(),
                      rng.gen::<f64>(),
                      rng.gen::<f64>()) * 2.0 - Vec3::new(1.0, 1.0, 1.0);
    }

    p
}

