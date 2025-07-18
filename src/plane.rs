use crate::{float::*, matrix::IDENTITY_MATRIX, object::*, prelude::Material, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Plane {}

impl Plane {
    pub fn normal_at(&self, _p: &Point) -> Point {
        vector(0.0, 1.0, 0.0)
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        if ray.direction.y.abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![t]
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
        let n1 = p.normal_at(&point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&point(-5.0, 0.0, 150.0));
        assert_eq!(n1, vector(0.0, 1.0, 0.0));
        assert_eq!(n2, vector(0.0, 1.0, 0.0));
        assert_eq!(n3, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane {};
        let r = ray(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane {};
        let r = ray(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane {};
        let r = ray(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane {};
        let r = ray(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
    }
}
