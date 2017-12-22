use vector::V;

use std::Ops;

pub struct M {
    pub m: [f64; 16];

    pub fn i() -> M {
        M { m: [1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0] }
    }

}
