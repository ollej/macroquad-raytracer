use crate::{bounds::*, intersection::*, material::*, matrix::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Clone, Debug)]
pub struct Group {
    pub children: Vec<Object>,
}

impl Group {
    pub fn new(children: Vec<Object>) -> Self {
        Self { children }
    }

    pub fn empty() -> Self {
        Self { children: vec![] }
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Result<Intersections, String> {
        if !object.bounding_box().intersects(ray) {
            return Ok(Intersections::empty());
        }

        let mut intersections = Intersections::empty();
        for child in self.children.iter() {
            intersections = intersections + child.intersect(ray)?;
        }
        Ok(intersections)
    }

    pub fn local_normal_at(&self, _p: &Point) -> Vector {
        unreachable!(
            "Normals are always computed by calling the concrete shape’s local_normal_at() method."
        )
    }

    pub fn add_child(&mut self, child: &mut Object) {
        self.children.push(child.to_owned());
    }
}

impl Bounds for Group {
    fn bounding_box(&self) -> BoundingBox {
        let mut bounding_box = BoundingBox::empty();

        for child in self.children.iter() {
            let cbox = child.bounding_box_in_parent_space();
            bounding_box = bounding_box + cbox;
        }

        bounding_box
    }
}

pub fn empty_group() -> Object {
    Object::new_group(IDENTITY_MATRIX, Material::default())
}

pub fn untransformed_group(children: &mut Vec<Object>) -> Object {
    let mut object = Object::new_group(IDENTITY_MATRIX, Material::default());
    children
        .iter_mut()
        .for_each(|child| object.add_child(child));
    object
}

pub fn group(transform: Matrix, children: &mut Vec<Object>) -> Object {
    let mut object = Object::new_group(transform, Material::default());
    children
        .iter_mut()
        .for_each(|child| object.add_child(child));
    object
}

#[cfg(test)]
mod test_chapter_14_group {
    use super::*;

    use crate::{shape::*, sphere::*};

    use std::f64::consts::PI;

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
        let mut s = Object::empty();
        let mut g = empty_group();
        g.add_child(&mut s);
        match g.shape {
            Shape::Group(group) => {
                assert_eq!(group.children.len(), 1);
                assert!(group.children.contains(&s));
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
        let mut s1 = sphere();
        let mut s2 = sphere();
        s2.set_transform(translation(0.0, 0.0, -3.0));
        let mut s3 = sphere();
        s3.set_transform(translation(5.0, 0.0, 0.0));
        let mut g = empty_group();
        g.add_child(&mut s1);
        g.add_child(&mut s2);
        g.add_child(&mut s3);

        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r).unwrap();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, s2);
        assert_eq!(xs[1].object, s2);
        assert_eq!(xs[2].object, s1);
        assert_eq!(xs[3].object, s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut s = sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));

        let mut g = empty_group();
        g.set_transform(scaling(2.0, 2.0, 2.0));
        g.add_child(&mut s);

        let r = ray(&point(10.0, 0.0, -10.0), &vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r).unwrap();

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let g1 = &mut empty_group();
        g1.set_transform(rotation_y(PI / 2.0));

        let g2 = &mut empty_group();
        g2.set_transform(scaling(2.0, 2.0, 2.0));
        g1.add_child(g2);

        let s = &mut sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));
        g2.add_child(s);

        let p = s.world_to_object(&point(-2.0, 0.0, -10.0)).unwrap();
        assert_eq!(p, point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let g1 = &mut empty_group();
        g1.set_transform(rotation_y(PI / 2.0));
        let g2 = &mut empty_group();
        g2.set_transform(scaling(1.0, 2.0, 3.0));
        g1.add_child(g2);
        let s = &mut sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));
        g2.add_child(s);
        let n = s
            .normal_to_world(&vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
            ))
            .unwrap();
        assert_eq!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    fn finding_the_normal_on_a_child_object() {
        let g1 = &mut empty_group();
        g1.set_transform(rotation_y(PI / 2.0));
        let g2 = &mut empty_group();
        g2.set_transform(scaling(1.0, 2.0, 3.0));
        g1.add_child(g2);
        let s = &mut sphere();
        s.set_transform(translation(5.0, 0.0, 0.0));
        g2.add_child(s);
        let n = s.normal_at(&point(1.7321, 1.1547, -5.5774)).unwrap();
        assert_eq!(n, vector(0.2857, 0.4286, -0.8571));
    }
}

#[cfg(test)]
mod test_chapter_14_group_bounds {
    use super::*;

    use crate::test_common::*;
    use crate::{cube::*, cylinder::*, sphere::*};

    #[test]
    fn groups_have_a_bounding_box_containing_all_children() {
        let mut g = empty_group();
        let c1 = &mut cube();
        c1.set_transform(translation(-1.0, -1.0, -1.0));
        let c2 = &mut cube();
        c2.set_transform(translation(1.0, 1.0, 1.0));
        g.add_child(c1);
        g.add_child(c2);
        assert_eq!(
            g.bounding_box(),
            bounding_box(&point(-2.0, -2.0, -2.0), &point(2.0, 2.0, 2.0))
        );
    }

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let s = &mut sphere();
        s.set_transform(translation(2.0, 5.0, -3.0) * scaling(2.0, 2.0, 2.0));
        let c = &mut cylinder(-2.0, 2.0, true);
        c.set_transform(translation(-4.0, -1.0, 4.0) * scaling(0.5, 1.0, 0.5));
        let mut shape = empty_group();
        shape.add_child(s);
        shape.add_child(c);
        let b = shape.bounding_box();
        assert_eq!(b.minimum, point(-4.5, -3.0, -5.0));
        assert_eq!(b.maximum, point(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_ray_group_doesnt_test_children_if_box_is_missed() {
        let child = &mut test_shape();
        let mut shape = empty_group();
        shape.add_child(child);
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 1.0, 0.0));
        let xs = shape.intersect(&r).unwrap();
        assert!(xs.is_empty());
    }

    #[test]
    fn intersecting_ray_group_tests_children_if_box_is_hit() {
        let child = &mut test_shape();
        let mut shape = empty_group();
        shape.add_child(child);
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let xs = shape.intersect(&r).unwrap();
        assert_eq!(xs.is_empty(), false);
    }
}
