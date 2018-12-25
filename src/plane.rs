use vector::Vec3;
use vector;
use ray::Ray;
use tests::*;
use random::*;

// standard plane representation of signed distance from origin +
// normal
pub struct Plane {
    pub d: f64,
    pub normal: Vec3
}

impl Plane {
    pub fn from_point_and_normal(point: &Vec3,
                                 normal: &Vec3) -> Plane {
        let unit_normal = vector::unit_vector(normal);
        Plane { d: -point.dot(&unit_normal), normal: unit_normal }
    }
    pub fn new(d: f64, normal: &Vec3) -> Plane {
        Plane { d: d, normal: vector::unit_vector(normal) }
    }

    pub fn eval(&self, p: &Vec3) -> f64 {
        vector::dot(&self.normal, p) + self.d
    }
                
    pub fn signed_distance(&self, p: &Vec3) -> f64 {
        self.eval(p)
    }

    pub fn intersect_ray(&self, ray: &Ray) -> Option<Vec3> {
        let num = self.eval(&ray.origin());
        let den = self.normal.dot(&ray.direction());

        if (den.abs() < 1e-8) {
            None
        } else {
            Some(ray.point_at_parameter(-num / den))
        }
    }
    
    // port of Graphics Gems III, Plane-to-Plane Intersection
    pub fn intersect(&self, other: &Plane) -> Option<Ray> {

        // int GetXLine(vect4 *pl1, vect4 *pl2, vect3 *xdir, vect3 *xpt)
        // {
        // float invdet;  /* inverse of 2x2 matrix determinant */
        // vect3 dir2;    /* holds the squares of the coordinates of xdir */

        let pl1  = &self.normal;
        let pl1w = self.d;
        let pl2  = &other.normal;
        let pl2w = other.d;
       
        let eps = 1.0e-8;

        let xdir = vector::cross(&self.normal, &other.normal);
        
        // Vect3Cross(*pl1, *pl2, *xdir)
        // dir2.x = xdir->x * xdir->x;
        // dir2.y = xdir->y * xdir->y;
        // dir2.z = xdir->z * xdir->z;

        let dir2 = Vec3::new(xdir.x() * xdir.x(),
                             xdir.y() * xdir.y(),
                             xdir.z() * xdir.z());

        (if dir2.z() > dir2.y() && dir2.z() > dir2.x() && dir2.z() > eps {
            Some((1.0 / xdir.z(),
                  Vec3::new(pl1.y() * pl2w - pl2.y() * pl1w,
                            pl2.x() * pl1w - pl1.x() * pl2w,
                            0.0)))
        } else if dir2.y() > dir2.x() && dir2.y() > eps {
            Some((-1.0 / xdir.y(),
                  Vec3::new(pl1.z() * pl2w - pl2.z() * pl1w,
                            0.0,
                            pl2.x() * pl1w - pl1.x() * pl2w)))
        } else if dir2.x() > eps {
            Some((1.0 / xdir.x(),
                  Vec3::new(0.0,
                            pl1.z() * pl2w - pl2.z() * pl1w,
                            pl2.y() * pl1w - pl1.y() * pl2w)))
        } else {
            None
        }).map(|v| {
            let invdet = v.0;
            let xpt = v.1;
            let invdet2 = 1.0 / (dir2.x() + dir2.y() + dir2.z()).sqrt();
            Ray::new(xpt * invdet, xdir * invdet2)
        })

        // if (dir2.z > dir2.y && dir2.z > dir2.x && dir2.z > MRG_ZERO)
        //     {
        //     /* then get a point on the XY plane */
        //     invdet = 1.f / xdir->z;
        //     /* solve < pl1.x * xpt.x + pl1.y * xpt.y = - pl1.w >
        //              < pl2.x * xpt.x + pl2.y * xpt.y = - pl2.w > */
        //     Vect3Init(pl1->y * pl2->w - pl2->y * pl1->w,
        //               pl2->x * pl1->w - pl1->x * pl2->w, 0.0, *xpt)
        //     }
        // else if (dir2.y > dir2.x && dir2.y > MRG_ZERO)
        //     {
        //     /* then get a point on the XZ plane */
        //     invdet = -1.f / xdir->y;	/*** correction ***/
        //     /* solve < pl1.x * xpt.x + pl1.z * xpt.z = -pl1.w >
        //              < pl2.x * xpt.x + pl2.z * xpt.z = -pl2.w > */
        //     Vect3Init(pl1->z * pl2->w - pl2->z * pl1->w, 0.f,
        //               pl2->x * pl1->w - pl1->x * pl2->w, *xpt)
        //     }
        // else if (dir2.x > MRG_ZERO)
        //     {
        //     /* then get a point on the YZ plane */
        //     invdet = 1.f / xdir->x;
        //     /* solve < pl1.y * xpt.y + pl1.z * xpt.z = - pl1.w >
        //              < pl2.y * xpt.y + pl2.z * xpt.z = - pl2.w > */
        //     Vect3Init(0.f, pl1->z * pl2->w - pl2->z * pl1->w,
        //               pl2->y * pl1->w - pl1->y * pl2->w, *xpt)
        //     }
        // else /* xdir is zero, then no point of intersection exists */
        //     return FALSE;
        // Vect3Muls(invdet, *xpt, *xpt)
        // invdet = 1.f / (float)sqrt(dir2.x + dir2.y + dir2.z);
        // Vect3Muls(invdet, *xdir, *xdir)
        // return TRUE;
        // }        
    }
}

//////////////////////////////////////////////////////////////////////////////

#[test]
fn it_works() {
    let plane1 = Plane::from_point_and_normal(
        &Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0,  1.0, 0.0));
    let plane2 = Plane::from_point_and_normal(
        &Vec3::new(1.0, 0.0, 1.0), &Vec3::new(0.0,  1.0, 0.0));
    let plane3 = Plane::from_point_and_normal(
        &Vec3::new(2.0, 0.0, 0.0), &Vec3::new(-1.0, 0.0, 0.0));

    assert!(within_eps_f(plane1.eval(&Vec3::new(0.0, 0.0, 0.0)), 0.0));
    assert!(within_eps_f(plane1.d, plane2.d));
    assert!(within_eps_f(plane1.signed_distance(&Vec3::new(0.0, 2.0, 0.0)),   2.0));
    assert!(within_eps_f(plane1.signed_distance(&Vec3::new(0.0, -2.0, 0.0)), -2.0));

    let ray = Ray::new(Vec3::new(3.0, 3.0, 0.0), Vec3::new(1.0, 1.0, 1.0));
    let isect = plane2.intersect_ray(&ray);
    assert!(isect.is_some());
    assert!(within_eps_f(plane2.eval(&isect.unwrap()), 0.0));

    let ray_intersection = plane2.intersect(&plane3);
    assert!(ray_intersection.is_some());
    for i in 0..100 {
        let u = rand_double();
        assert!(within_eps_f(plane2.eval(&ray_intersection.unwrap().point_at_parameter(u)), 0.0));
        assert!(within_eps_f(plane3.eval(&ray_intersection.unwrap().point_at_parameter(u)), 0.0));
    }
}
