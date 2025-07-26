use core::f64;

use crate::{
    bounds::*, float::*, intersection::*, material::*, matrix::IDENTITY_MATRIX, object::*, ray::*,
    tuple::*,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Plane {}

impl Plane {
    pub fn local_normal_at(&self, _p: &Point) -> Vector {
        vector(0.0, 1.0, 0.0)
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        if ray.direction.y.abs() < EPSILON {
            Intersections::empty()
        } else {
            let t = -ray.origin.y / ray.direction.y;
            let xs = vec![t];
            Intersections::from_object(xs, object)
        }
    }
}

impl Bounds for Plane {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            minimum: point(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY),
            maximum: point(f64::INFINITY, 0.0, f64::INFINITY),
        }
    }
}

pub fn plane() -> Object {
    Object::new_plane(IDENTITY_MATRIX, Material::default())
}

#[cfg(test)]
mod test_chapter_9_planes {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane {};
        let n1 = p.local_normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn local_intersect_with_a_ray_parallel_to_the_plane() {
        let p = plane();
        let r = ray(&point(0.0, 10.0, 0.0), &vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r).unwrap();
        assert!(xs.is_empty());
    }

    #[test]
    fn local_intersect_with_a_coplanar_ray() {
        let p = plane();
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r).unwrap();
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = plane();
        let r = ray(&point(0.0, 1.0, 0.0), &vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = plane();
        let r = ray(&point(0.0, -1.0, 0.0), &vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
    }
}

#[cfg(test)]
mod test_chapter_14_planes_bounds {
    use core::f64;

    use super::*;

    #[test]
    fn planes_have_a_bounding_box_to_infinity() {
        let p = plane();
        let b = p.bounding_box();
        assert_eq!(b.minimum, point(f64::NEG_INFINITY, 0.0, f64::NEG_INFINITY));
        assert_eq!(b.maximum, point(f64::INFINITY, 0.0, f64::INFINITY));
    }
}
