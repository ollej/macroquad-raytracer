use crate::{
    float::*, intersection::*, matrix::IDENTITY_MATRIX, object::*, prelude::Material, ray::*,
    tuple::*,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Plane {}

impl Plane {
    pub fn normal_at(&self, p: &Point) -> Point {
        vector(0.0, 1.0, 0.0)
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Float, Float)> {
        None
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
}
