use camera::Camera;
use background::Background;
use hitable::Hitable;

use std::vec::Vec;

pub struct Scene {
    pub camera: Camera,
    pub background: Box<Background>,
    pub object_list: Vec<Box<Hitable>>
}

impl Scene {
    pub fn new(camera: &Camera,
               background: Box<Background>,
               object_list: Vec<Box<Hitable>>) -> Scene {
        Scene {
            camera: *camera,
            background: background,
            object_list: object_list
        }
    }
}
