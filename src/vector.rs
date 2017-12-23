use std::ops;

#[derive(Debug, Copy, Clone)]
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

//////////////////////////////////////////////////////////////////////////////

#[test]
fn it_doesnt_smoke() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let mut v2 = Vec3::new(0.0, 1.0, 0.0);
    
    println!("v1 is {:?}", v1);
    println!("v1 + v2 is {:?}", v1 + v2); // this is a little goofy, but ok
    println!("v1 - v2 is {:?}", v1 - v2);
    println!("v1 * v1 is {:?}", v1 * v1);

    println!("v1.dot(v2) is {:?}", v1.dot(&v2));

    println!("v1 * v2 is {:?}", v1 * v2);
    println!("v2 * 2 is {:?}", v2 * 2.0);
    println!("2 * v2 is {:?}", 2.0 * v2);

    println!("v1 / v2 is {:?}", v1 / v2);
    println!("v2 / 2 is {:?}", v2 / 2.0);
    println!("2 / v2 is {:?}", 2.0 / v2);

    println!("v2[0] is {:?}", &v2[0]);

    (&mut v2)[0] = 5.0;

    println!("v2[0] is {:?}", &v2[0]);
    println!("-v2 is {:?}", -v2);
    println!("v2.length() is {:?}", v2.length());
    println!("v2.length_squared() is {:?}", v2.length_squared());
    println!("v1 cross v2 is {:?}", cross(v1, v2));
}
