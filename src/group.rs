use crate::{float::*, material::*, matrix::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    pub children: Vec<Object>,
}

impl Group {
    pub fn new() -> Self {
        Self { children: vec![] }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Float> {
        vec![]
    }

    pub fn local_normal_at(&self, _p: &Point) -> Vector {
        unreachable!()
    }
}

pub fn default_group() -> Object {
    Object::new_group(IDENTITY_MATRIX, Material::default())
}

pub fn group(transform: Matrix, material: Material) -> Object {
    Object::new_group(transform, material)
}

#[cfg(test)]
mod test_chapter_14_group {
    use super::*;

    use crate::shape::*;

    #[test]
    fn creating_a_new_group() {
        let g = default_group();
        assert_eq!(g.transform, IDENTITY_MATRIX);
        match g.shape {
            Shape::Group(group) => assert_eq!(group.children.len(), 0),
            _ => panic!("Shape should be a group"),
        };
    }
}
