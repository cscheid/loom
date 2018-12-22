use vector::*;

pub trait Background {
    fn get_background(&self, unit_direction: &Vec3) -> Vec3;
}

pub struct Sky {}
pub struct OverheadLight {}
pub struct Constant {
    pub color: Vec3
}

impl Background for Sky {
    fn get_background(&self, unit_direction: &Vec3) -> Vec3 {
        lerp(&Vec3::new(1.0, 1.0, 1.0),
             &Vec3::new(0.5, 0.7, 1.0),
             0.5*(unit_direction.y() + 1.0))
    }
}

impl Background for Constant {
    fn get_background(&self, _: &Vec3) -> Vec3 {
        self.color
    }
}

impl Background for OverheadLight {
    fn get_background(&self, unit_direction: &Vec3) -> Vec3 {
        if unit_direction.y() > 0.0 { 
            Vec3::new(unit_direction.y(),
                      unit_direction.y(),
                      unit_direction.y())
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

pub fn overhead_light() -> OverheadLight {
    OverheadLight {}
}

pub fn sky() -> Sky {
    Sky {}
}

pub fn constant(color: Vec3) -> Constant {
    Constant {
        color: color
    }
}
