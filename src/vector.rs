// V for Vector, since Vec is already taken :(

use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct V {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl V {
    pub fn new(x: f64, y: f64, z: f64) -> V {
        V { x: x, y: y, z: z }
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a, 'b> ops::Add<&'b V> for &'a V {
    type Output = V;

    fn add(self, _rhs: &'b V) -> V {
        V { x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z }
    }
}

impl<'a, 'b> ops::Sub<&'b V> for &'a V {
    type Output = V;

    fn sub(self, _rhs: &'b V) -> V {
        V { x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z }
    }
}

impl<'a, 'b> ops::Mul<&'b V> for &'a V {
    type Output = f64;
    fn mul(self, _rhs: &'b V) -> f64 {
        self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z
    }
}
