use crate::{intersection::*, material::*, matrix::*, object::*, ray::*, tuple::*};

use std::sync::Arc;

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    pub children: Vec<Arc<Object>>,
}

impl Group {
    pub fn new(children: Vec<Object>) -> Self {
        Self {
            children: children.iter().map(|i| Arc::new(i.to_owned())).collect(),
        }
    }

    pub fn local_intersect(&self, ray: &Ray, _object: &Object) -> Result<Intersections, String> {
        Ok(Intersections::new(
            self.children
                .iter()
                .flat_map(|child| child.intersect(ray))
                .map(|intersections| intersections.inner().clone())
                .flatten()
                .collect(),
        ))
    }

    pub fn local_normal_at(&self, _p: &Point) -> Vector {
        unreachable!()
    }
}

pub fn empty_group() -> Object {
    Object::new_group(IDENTITY_MATRIX, Material::default(), vec![])
}

pub fn untransformed_group(children: Vec<Object>) -> Object {
    Object::new_group(IDENTITY_MATRIX, Material::default(), children)
}

pub fn group(transform: Matrix, children: Vec<Object>) -> Object {
    Object::new_group(transform, Material::default(), children)
}

#[cfg(test)]
mod test_chapter_14_group {
    use super::*;

    use crate::{shape::*, sphere::*};

    #[test]
    fn creating_a_new_group() {
        let g = empty_group();
        assert_eq!(g.transform, IDENTITY_MATRIX);
        match g.shape {
            Shape::Group(group) => assert_eq!(group.children.len(), 0),
            _ => panic!("Shape should be a group"),
        };
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let s = Object::empty();
        let g = Object::new_group(IDENTITY_MATRIX, Material::default(), vec![s.clone()]);
        match g.shape {
            Shape::Group(group) => {
                assert_eq!(group.children.len(), 1);
                assert!(group.children.contains(&Arc::new(s)));
                // assert group.parent = g
            }
            _ => panic!("Shape should be a group"),
        };
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = empty_group();
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r).unwrap();
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let s1 = sphere();
        let mut s2 = sphere();
        s2.set_transform(&translation(0.0, 0.0, -3.0));
        let mut s3 = sphere();
        s3.set_transform(&translation(5.0, 0.0, 0.0));
        let g = Object::new_group(
            IDENTITY_MATRIX,
            Material::default(),
            vec![s1.clone(), s2.clone(), s3],
        );
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r).unwrap();
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
        assert_eq!(xs[2].object, s1);
        assert_eq!(xs[3].object, s1);
    }
}
