use vector::Vec3;
use random::*;
    
//////////////////////////////////////////////////////////////////////////////

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    
    while p.dot(&p) >= 1.0 {
        p = Vec3::new(rand_double() * 2.0 - 1.0,
                      rand_double() * 2.0 - 1.0,
                      rand_double() * 2.0 - 1.0);
    }

    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p = Vec3::new(1.0, 1.0, 0.0);

    while p.dot(&p) >= 1.0 {
        p = Vec3::new(rand_double() * 2.0 - 1.0,
                      rand_double() * 2.0 - 1.0,
                      0.0);
    }

    p
}

