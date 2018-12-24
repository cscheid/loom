use aabb::AABB;
use aabb;
use hitable::*;
use lambertian::*; // for testing
use material::Material;
use ray::*;
use std::option::Option;
use tests::*;
use vector::*;

use rand;
use rand::Rng;
use std::cmp::Ordering;

//////////////////////////////////////////////////////////////////////////////

pub struct Triangle {
    pub vertices: [Vec3; 3],
}

//////////////////////////////////////////////////////////////////////////////
// This is super annoying because it's effectively a copy-paste from bvh.rs,
// but I want to be careful not to allocate too much here.

// yeah, this will do weird things with NaNs in the picture.
#[inline]
fn ffcmp(a: f64, b: f64) -> Ordering {
    if      a < b { Ordering::Less }
    else if a > b { Ordering::Greater }
    else          { Ordering::Equal }
}

struct MeshBVH {
    pub min_ix: usize,
    pub max_ix: usize,
    pub left: Option<Box<MeshBVH>>,
    pub right: Option<Box<MeshBVH>>,
    pub bbox: AABB
}

pub struct TriangleMesh {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub material: Box<Material>,
    bvh: Box<MeshBVH>
}

fn build_triangle_aabb(tris: &[Triangle], min_ix: usize, max_ix: usize) -> AABB
{
    let mut result = AABB::zero();
    for i in min_ix..max_ix {
        result.update(&tris[i].vertices[0]);
        result.update(&tris[i].vertices[1]);
        result.update(&tris[i].vertices[2]);
    }
    result
}

const MIN_LENGTH: usize = 32;

fn build_mesh_bvh(tris: &mut Vec<Triangle>, min_ix: usize, max_ix: usize) -> Option<Box<MeshBVH>> {
    let len = max_ix - min_ix;
    if len <= MIN_LENGTH {
        Some(Box::new(MeshBVH {
            min_ix: min_ix,
            max_ix: max_ix,
            left: None,
            right: None,
            bbox: build_triangle_aabb(tris, min_ix, max_ix)
        }))
    } else {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0, 3);
        tris[min_ix..max_ix].sort_unstable_by(|t1, t2| {
            let b1 = AABB::from_points(&t1.vertices);
            let b2 = AABB::from_points(&t2.vertices);
            ffcmp(b1.min()[axis], b2.min()[axis])
        });
        let median_ix = len / 2;
        let left_bvh = build_mesh_bvh(tris, min_ix, min_ix+median_ix);
        let right_bvh = build_mesh_bvh(tris, min_ix+median_ix, max_ix);
        let left_bbox = left_bvh.as_ref().unwrap().bbox;
        let right_bbox = right_bvh.as_ref().unwrap().bbox;
        Some(Box::new(MeshBVH {
            min_ix: min_ix,
            max_ix: max_ix,
            left: left_bvh,
            right: right_bvh,
            bbox: aabb::surrounding_box(&left_bbox, &right_bbox)
        }))
    }
}

impl<'a> TriangleMesh {
    pub fn new(mat: Box<Material>,
               verts: Vec<Vec3>,
               indices: Vec<usize>) -> TriangleMesh {
        let mut tris = Vec::new();
        for i in 0..indices.len()/3 {
            let v1 = indices[3*i];
            let v2 = indices[3*i+1];
            let v3 = indices[3*i+2];
            tris.push(Triangle {
                vertices: [verts[v1], verts[v2], verts[v3]]
            });
        }
        let l = tris.len();
        let bvh = build_mesh_bvh(&mut tris, 0, l).unwrap();
        TriangleMesh {
            vertices: verts,
            triangles: tris,
            material: mat,
            bvh: bvh
        }
    }

    fn hit_bvh(&'a self, current_node: &Box<MeshBVH>,
               r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        if current_node.bbox.hit(r, t_min, t_max) {
            match &current_node.left {
                &None => {
                    let mut result = None;
                    for i in current_node.min_ix..current_node.max_ix {
                        if let Some(hit_t) = self.triangles[i].hit(r) {
                            if hit_t >= t_min && hit_t <= t_max {
                                match result {
                                    None => {
                                        result = Some((hit_t, i));
                                    },
                                    Some((old_t, _old_i)) => {
                                        if hit_t < old_t {
                                            result = Some((hit_t, i));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match result {
                        None => None,
                        Some((hit_t, i)) => {
                            let tri = &self.triangles[i];
                            let n = unit_vector(&cross(
                                &(tri.vertices[1] - tri.vertices[0]),
                                &(tri.vertices[2] - tri.vertices[0])));
                            Some(HitRecord::hit(hit_t,
                                                r.point_at_parameter(hit_t),
                                                n,
                                                &*self.material))
                        }
                    }
                },
                &Some(ref left_node) => {
                    let right_node = &current_node.right.as_ref().unwrap();
                    let hit_left  =  self.hit_bvh(left_node,  r, t_min, t_max);
                    let hit_right =  self.hit_bvh(right_node, r, t_min, t_max);
                    match (hit_left, hit_right) {
                        (None, None) => None,
                        (None, right_rec) => {
                            right_rec
                        },
                        (left_rec, None) => {
                            left_rec
                        },
                        (Some(left_rec), Some(right_rec)) => {
                            if left_rec.t < right_rec.t {
                                Some(left_rec)
                            } else {
                                Some(right_rec)
                            }
                        }
                    }
                }
            }
        } else {
            None
        }
    }
}

// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
impl Triangle {
    pub fn hit(&self, r: &Ray) -> Option<f64> {
        const EPSILON: f64 = 0.0000001; 
        let vertex0 = &self.vertices[0];
        let vertex1 = &self.vertices[1];
        let vertex2 = &self.vertices[2];
        // Vector3D edge1, edge2, h, s, q;
        // float a,f,u,v;
        let edge1 = *vertex1 - *vertex0;
        let edge2 = *vertex2 - *vertex0;
        let h = cross(&r.direction(), &edge2);
        let a = edge1.dot(&h);
        if a > -EPSILON && a < EPSILON {
            return None;
        }
        let f = 1.0/a;
        let s = r.origin() - *vertex0;
        let u = f * s.dot(&h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = cross(&s, &edge1);
        let v = f * r.direction().dot(&q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let t = f * edge2.dot(&q);
        if t > EPSILON { // ray intersection
            return Some(t);
        } else {
            return None;
        }
    }
}

impl Hitable for TriangleMesh {
    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bvh.bbox)
    }
   
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        self.hit_bvh(&self.bvh, r, t_min, t_max)
    }
}

#[test]
fn it_works() {
    let verts = vec![Vec3::new(0.0, 0.0, 0.0),
                     Vec3::new(1.0, 0.0, 0.0),
                     Vec3::new(0.0, 1.0, 0.0)];
    let indices = vec![0, 1, 2];
    let mesh = TriangleMesh::new(Lambertian::new(&Vec3::new(1.0, 1.0, 1.0)),
                                 verts,
                                 indices);

    let ray1 = Ray::new(Vec3::new(0.25, 0.25, -1.0), Vec3::new(0.0, 0.0, 1.0));
    let ray2 = Ray::new(Vec3::new(0.75, 0.75, -1.0), Vec3::new(0.0, 0.0, 1.0));

    assert!(mesh.hit(&ray1, 0.00001, 1e30).is_some());
    assert!(mesh.hit(&ray2, 0.00001, 1e30).is_none());

    let hr = mesh.hit(&ray1, 0.00001, 1e30);
    assert!(hr.is_some());
    assert!(within_eps(&hr.unwrap().normal, &Vec3::new(0.0, 0.0, 1.0)));
}
