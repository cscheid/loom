use vector;
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

pub fn random_3d_direction() -> Vec3 {
    vector::unit_vector(&random_in_unit_sphere())
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

pub fn t_stat(itor: &mut std::iter::Iterator<Item=f64>,
              mean: f64) -> f64
{
    let sufficient = itor.fold((0.0, 0.0, 0.0), |acc, next| {
        (acc.0+1.0, acc.1+next, acc.2+next*next)
    });
    let n   = sufficient.0;
    let ex  = sufficient.1/n;
    let exx = sufficient.2 * sufficient.2/n;

    (ex - mean) / (exx.sqrt() / (n as f64).sqrt())
}

pub fn avstdev(itor: &mut std::iter::Iterator<Item=f64>) ->
    (f64, f64)
{
    let sufficient = itor.fold((0.0, 0.0, 0.0), |acc, next| {
        (acc.0+1.0, acc.1+next, acc.2+next*next)
    });
    let n   = sufficient.0;
    let ex  = sufficient.1/n;
    let exx = sufficient.2 * sufficient.2/n;
    (ex, exx - ex * ex)
}

#[test]
fn it_works()
{
    let mut itor1 = (0..10000).map(|_| {
        random_in_unit_disk().x()
    });
    let mut itor2 = (0..10000).map(|_| {
        random_in_unit_disk().y()
    });
    let stats1 = avstdev(&mut itor1);
    let stats2 = avstdev(&mut itor2);
    println!("random unit disk sampling average in x: {}", stats1.0);
    println!("random unit disk sampling average in y: {}", stats2.0);
}
