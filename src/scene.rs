use camera::Camera;
use background::Background;
use hitable::Hitable;

use std::vec::Vec;

pub struct Scene {
    pub camera: Camera,
    pub background: Box<Background + Send + Sync>,
    pub object_list: Vec<Box<Hitable + Send + Sync>>
}

impl Scene {
    pub fn new(camera: &Camera,
               background: Box<Background + Send + Sync>,
               object_list: Vec<Box<Hitable + Send + Sync>>) -> Scene {
        Scene {
            camera: *camera,
            background: background,
            object_list: object_list
        }
    }
}
