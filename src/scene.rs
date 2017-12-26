use camera::Camera;
use background::Background;
use hitable::Hitable;

use std::rc::Rc;
use std::vec::Vec;

pub struct Scene {
    pub camera: Camera,
    pub background: Rc<Background>,
    pub object_list: Vec<Box<Hitable>>
}

impl Scene {
    pub fn new(camera: &Camera,
               background: Rc<Background>,
               object_list: Vec<Box<Hitable>>) -> Scene {
        Scene {
            camera: *camera,
            background: background,
            object_list: object_list
        }
    }
}
