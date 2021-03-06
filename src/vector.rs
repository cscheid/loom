use std::ops;

// testing imports
#[allow(unused_imports)]
use tests::*;
#[allow(unused_imports)]
use rand::Rng;

#[derive(Serialize, PartialEq, Deserialize, Debug, Copy, Clone)]
pub struct Vec3 {
    e: [f64; 3]
}

impl Vec3 {
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    #[inline]
    pub fn zero() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    #[inline]
    pub fn x(&self) -> f64 { self.e[0] }
    #[inline]
    pub fn y(&self) -> f64 { self.e[1] }
    #[inline]
    pub fn z(&self) -> f64 { self.e[2] }
    #[inline]

    #[inline]
    pub fn r(&self) -> f64 { self.e[0] }
    #[inline]
    pub fn g(&self) -> f64 { self.e[1] }
    #[inline]
    pub fn b(&self) -> f64 { self.e[2] }

    #[inline]
    pub fn dot(&self, _rhs: &Vec3) -> f64 {
        self.e[0] * _rhs.e[0] + self.e[1] * _rhs.e[1] + self.e[2] * _rhs.e[2]
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.dot(&self)
    }

    #[inline]
    pub fn set(&mut self, _rhs: &Vec3) {
        self.e[0] = _rhs[0];
        self.e[1] = _rhs[1];
        self.e[2] = _rhs[2];
    }
    
}

//////////////////////////////////////////////////////////////////////////////

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index<'a>(&'a self, index: usize) -> &'a f64 {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut f64 {
        &mut self.e[index]
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] + _rhs.e[0],
                   self.e[1] + _rhs.e[1],
                   self.e[2] + _rhs.e[2]] }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] - _rhs.e[0],
                   self.e[1] - _rhs.e[1],
                   self.e[2] - _rhs.e[2]] }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] * _rhs.e[0],
                   self.e[1] * _rhs.e[1],
                   self.e[2] * _rhs.e[2]] }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 { e: [self.e[0] * _rhs,
                   self.e[1] * _rhs,
                   self.e[2] * _rhs] }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [_rhs.e[0] * self,
                   _rhs.e[1] * self,
                   _rhs.e[2] * self] }
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [self.e[0] / _rhs.e[0],
                   self.e[1] / _rhs.e[1],
                   self.e[2] / _rhs.e[2]] }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 { e: [self.e[0] / _rhs,
                   self.e[1] / _rhs,
                   self.e[2] / _rhs] }
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, _rhs: Vec3) -> Vec3 {
        Vec3 { e: [self / _rhs.e[0],
                   self / _rhs.e[1],
                   self / _rhs.e[2]] }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {e: [-self.e[0],
                  -self.e[1],
                  -self.e[2]] }
    }
}

pub fn tangent_space(&normal: &Vec3) -> (Vec3, Vec3) {
    let mut t = cross(&normal, &Vec3::new(1.0, 0.0, 0.0));
    if t.length() < 0.1 {
        t = cross(&normal, &Vec3::new(0.0, 1.0, 0.0));
        if t.length() < 0.1 {
            t = cross(&normal, &Vec3::new(0.0, 0.0, 1.0));
        }
        if t.length() < 0.1 {
            panic!("You should have sent a non-zero vector");
        }
    }
    let u_vec = unit_vector(&t);

    (u_vec, unit_vector(&cross(&normal, &u_vec)))
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    *v / v.length()
}

pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
    Vec3::new(v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
              v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0])
}

pub fn lerp(v1: &Vec3, v2: &Vec3, u: f64) -> Vec3 {
    (1.0-u) * *v1 + u * *v2
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
    v1.dot(v2)
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - ((2.0 * dot(v, n)) * *n)
}

pub fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = unit_vector(v);
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - (*n) * dt) - (*n) * discriminant.sqrt())
    } else {
        None
    }
}

// rotate v about k by theta
// assumes k is unit length!!!
pub fn rotate(v: &Vec3, k: &Vec3, theta: f64) -> Vec3 {
    let sin_cos = theta.sin_cos();
    let sin = sin_cos.0;
    let cos = sin_cos.1;

    // v_rot = v cos + (k cross v) sin + k (k dot v) (1 - cos)
    (*v) * cos + cross(k, v) * sin + (*k * dot(k, v)) * (1.0 - cos)
}

// rotate v about k by theta, given sin(theta) and cos(theta)
// assumes k is unit length!!!
pub fn rotate_sincos(v: &Vec3, k: &Vec3, sin: f64, cos: f64) -> Vec3 {
    // let sin_cos = theta.sin_cos();
    // let sin = sin_cos.0;
    // let cos = sin_cos.1;

    // v_rot = v cos + (k cross v) sin + k (k dot v) (1 - cos)
    (*v) * cos + cross(k, v) * sin + (*k * dot(k, v)) * (1.0 - cos)
}

//////////////////////////////////////////////////////////////////////////////
// interpreting vec3 as rgb color

pub fn luminance(v: &Vec3) -> f64 {
    // https://en.wikipedia.org/wiki/Relative_luminance
    return dot(v, &Vec3::new(0.2126, 0.7152, 0.0722));
}

//////////////////////////////////////////////////////////////////////////////

#[test]
fn it_works() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let mut rng = rand::thread_rng();

    assert!(within_eps(&(v1 + v2), &Vec3::new(1.0, 1.0, 0.0)));
    assert!(within_eps(&(v1 + v2), &Vec3::new(1.0, 1.0, 0.0)));

    for _ in 0..100 {
        let r1 = random_vec();
        let r2 = random_vec();
        let rf = rng.gen::<f64>();
        
        assert!(within_eps(&(r1 + r2), &(r2 + r1)));
        assert!(within_eps(&(r1 * r2), &(r2 * r1)));
        assert!(within_eps_f(r1.dot(&r2), r2.dot(&r1)));
        assert!(within_eps_f(r1.dot(&(r2 * rf)), (r1 * rf).dot(&r2)));
        assert!(within_eps_f(r1.dot(&(r2 / rf)), (r1 / rf).dot(&r2)));
        assert!(within_eps_f(r1.length() * rf, (r1 * rf).length()));
        assert!(within_eps_f(r1.length() / rf, (r1 / rf).length()));
        assert!(within_eps_f(r1.length() * r1.length(), r1.length_squared()));
        assert!(within_eps(&(r1 * -1.0), &(-r1)));
    }

    for _ in 0..100 {
        let r1 = unit_vector(&random_vec());
        let r2 = unit_vector(&random_vec());

        let v = cross(&r1, &r2);
        let axis = unit_vector(&v);
        let cos_angle = dot(&r1, &r2) / (r1.length() * r2.length());
        assert!(within_eps(&rotate(&r1, &axis, cos_angle.acos()), &r2));
    }
    // FIXME: test cross product
    // println!("v1 cross v2 is {:?}", cross(&v1, &v2));
}
