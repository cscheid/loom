use serde_json::*;
use background::*;
use camera::*;
use vector::*;
use dielectric::*;
use hitable::*;
use hitable_list::*;
use lambertian::*;
use material::*;
use metal::*;
use mixture::*;
use scene::*;
use sphere::*;

use std::option::*;
use std::rc::Rc;
use std::vec::*;
use std::iter::*;

//////////////////////////////////////////////////////////////////////////////

pub fn deserialize_vec3(v: &Value) -> Option<Vec3>
{
    match v {
        &Value::Array(ref ns) => {
            if ns.len() != 3 {
                eprintln!("len != 3");
                None
            } else if ns.iter().any(|x| !x.is_number()) {
                eprintln!("some are not number");
                None
            } else {
                let x = ns[0].as_f64().unwrap();
                let y = ns[1].as_f64().unwrap();
                let z = ns[2].as_f64().unwrap();
                Some(Vec3::new(x, y, z))
            }
        }
        _ => {
            eprintln!("Passed object not an array");
            None
        }
    }
}

pub fn deserialize_camera(v: &Value) -> Option<Camera>
{
    match v {
        &Value::Object(ref m) => {
            let look_from  = deserialize_vec3(&m["look_from"]);
            let look_at    = deserialize_vec3(&m["look_at"]);
            let vup        = deserialize_vec3(&m["vup"]);
            let vfov       = m["vfov"].as_f64();
            let aspect     = m["aspect"].as_f64();
            let aperture   = m["aperture"].as_f64();
            let focus_dist = m["focus_dist"].as_f64();

            if look_from.is_none() || look_at.is_none() ||
                vup.is_none() || vfov.is_none() ||
                aspect.is_none() || aperture.is_none() ||
                focus_dist.is_none() {
                None
            } else {
                    Some(Camera::new(&look_from.unwrap(), &look_at.unwrap(), &vup.unwrap(),
                                     vfov.unwrap(), aspect.unwrap(),
                                     aperture.unwrap(), focus_dist.unwrap()))
            }
        },
        _ => None
    }
}

pub fn deserialize_dielectric(v: &Value) -> Option<Rc<Material>>
{
    match v {
        &Value::Object(ref m) => {
            m["refraction_index"]
                .as_f64()
                .map(|ri| Dielectric::new(ri))
        },
        _ => None
    }
}

pub fn deserialize_lambertian(v: &Value) -> Option<Rc<Material>>
{
    match v {
        &Value::Object(ref m) => {
            deserialize_vec3(&m["albedo"])
                .map(|a| Lambertian::new(&a))
        },
        _ => None
    }
}

pub fn deserialize_metal(v: &Value) -> Option<Rc<Material>>
{
    match v {
        &Value::Object(ref m) => {
            deserialize_vec3(&m["albedo"])
                .map(|a| Metal::new(&a))
        },
        _ => None
    }
}

pub fn deserialize_mixture(v: &Value) -> Option<Rc<Material>>
{
    match v {
        &Value::Object(ref m) => {
            let mat_1 = deserialize_material(&m["mat_1"]);
            let mat_2 = deserialize_material(&m["mat_2"]);
            let u = m["u"].as_f64();
            if mat_1.is_none() || mat_2.is_none() ||
                u.is_none() {
                    None
                } else {
                    Some(Mixture::new(mat_1.unwrap(),
                                      mat_2.unwrap(),
                                      u.unwrap()))
                }
        },
        _ => None
    }
}

pub fn deserialize_sphere(v: &Value) -> Option<Box<Hitable>>
{
    match v {
        &Value::Object(ref m) => {
            let center = deserialize_vec3(&m["center"]);
            let radius = m["radius"].as_f64();
            let material = deserialize_material(&m["material"]);
            if center.is_none() || radius.is_none() ||
                material.is_none() {
                    None
                } else {
                    Some(Box::new(Sphere::new(center.unwrap(),
                                              radius.unwrap(),
                                              material.unwrap())))
                }
        },
        _ => None
    }
}

pub fn deserialize_hitable_list(v: &Value) -> Option<Box<Hitable>>
{
    match v {
        &Value::Array(ref m) => {
            let mut objs = Vec::from_iter(m.iter().map(deserialize_hitable));
            if objs.iter().any(|x| !x.is_none()) {
                None
            } else {
                Some(Box::new(HitableList::new(
                    objs.drain(..)
                        .map(|x| x.unwrap())
                        .collect())))
            }
        },
        _ => None
    }
}

//////////////////////////////////////////////////////////////////////////////

pub fn deserialize_background(v: &Value) -> Option<Rc<Background>>
{
    match v {
        &Value::String(ref m) => {
            if m == &"sky".to_string() {
                Some(Rc::new(sky()))
            } else if m == &"overhead_light".to_string() {
                Some(Rc::new(overhead_light()))
            } else {
                None
            }
        }
        _ => None
    }
}

pub fn deserialize_material(v: &Value) -> Option<Rc<Material>>
{
    match v {
        &Value::Object(ref m) => {
            let class = m["class"].as_str();
            let object = &m["object"];
            if class.is_none() {
                None
            } else {
                let name = class.unwrap();
                if name == "dielectric" {
                    deserialize_dielectric(object)
                } else if name == "lambertian" {
                    deserialize_lambertian(object)
                } else if name == "metal" {
                    deserialize_metal(object)
                } else if name == "mixture" {
                    deserialize_mixture(object)
                } else {
                    None
                }
            }
        },
        _ => None
    }
}

pub fn deserialize_hitable(v: &Value) -> Option<Box<Hitable>>
{
    match v {
        &Value::Object(ref m) => {
            let class = m["class"].as_str();
            let object = &m["object"];
            if class.is_none() {
                None
            } else {
                let name = class.unwrap();
                if name == "sphere" {
                    deserialize_sphere(object)
                } else if name == "hitable_list" {
                    deserialize_hitable_list(object)
                } else {
                    None
                }
            }
        },
        _ => None
    }
}

pub fn deserialize_scene_object_list(v: &Value) -> Option<Vec<Box<Hitable>>>
{
    match v {
        &Value::Array(ref m) => {
            let mut objs = Vec::from_iter(m.iter().map(deserialize_hitable));
            if objs.iter().any(|x| x.is_none()) {
                None
            } else {
                Some(objs.drain(..).map(|x| x.unwrap()).collect())
            }
        },
        _ => None
    }
}

pub fn deserialize_scene(v: &Value) -> Option<Scene>
{
    match v {
        &Value::Object(ref m) => {
            let list = deserialize_scene_object_list(&m["object_list"]);
            let camera = deserialize_camera(&m["camera"]);
            let background = deserialize_background(&m["background"]);
            if list.is_none() || camera.is_none() ||
                background.is_none() {
                    None
                } else {
                    Some(Scene::new(&camera.unwrap(),
                                    background.unwrap(),
                                    list.unwrap()))
                }
        },
        _ => None
    }
}
