use crate::{
    bounds::*, color::*, cone::*, cube::*, cylinder::*, float::*, group::*, intersection::*,
    light::*, material::*, matrix::*, plane::*, ray::*, shape::*, sphere::*, triangle::*, tuple::*,
};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Object {
    pub transform: Matrix,
    pub inverse_transform: Matrix,
    pub material: Material,
    pub shape: Shape,
    pub parent: Option<Arc<Object>>,
}

impl Object {
    pub fn empty() -> Result<Self, String> {
        Self::new(IDENTITY_MATRIX)
    }

    pub fn new(transform: Matrix) -> Result<Self, String> {
        Self::new_sphere(transform, Material::default())
    }

    pub fn new_sphere(transform: Matrix, material: Material) -> Result<Self, String> {
        let shape = Shape::Sphere(Sphere {});
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_plane(transform: Matrix, material: Material) -> Result<Self, String> {
        let shape = Shape::Plane(Plane {});
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_cube(transform: Matrix, material: Material) -> Result<Self, String> {
        let shape = Shape::Cube(Cube {});
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_cylinder(
        minimum: Float,
        maximum: Float,
        closed: bool,
        transform: Matrix,
        material: Material,
    ) -> Result<Self, String> {
        let shape = Shape::Cylinder(Cylinder::new(minimum, maximum, closed));
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_cone(
        minimum: Float,
        maximum: Float,
        closed: bool,
        transform: Matrix,
        material: Material,
    ) -> Result<Self, String> {
        let shape = Shape::Cone(Cone::new(minimum, maximum, closed));
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_triangle(
        p1: Point,
        p2: Point,
        p3: Point,
        transform: Matrix,
        material: Material,
    ) -> Result<Self, String> {
        let shape = Shape::Triangle(Triangle::new(p1, p2, p3));
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_smooth_triangle(
        p1: Point,
        p2: Point,
        p3: Point,
        n1: Vector,
        n2: Vector,
        n3: Vector,
        transform: Matrix,
        material: Material,
    ) -> Result<Self, String> {
        let shape = Shape::SmoothTriangle(SmoothTriangle::new(p1, p2, p3, n1, n2, n3));
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn new_group(transform: Matrix, material: Material) -> Result<Self, String> {
        let shape = Shape::Group(Group::empty());
        Ok(Self {
            transform,
            inverse_transform: transform.inverse()?,
            material,
            shape,
            parent: None,
        })
    }

    pub fn set_transform(&mut self, matrix: Matrix) -> Result<(), String> {
        self.transform = matrix;
        self.inverse_transform = matrix.inverse()?;
        self.shape.update_parents(&mut self.clone());
        Ok(())
    }

    pub fn set_material(&mut self, material: &Material) {
        self.material = material.to_owned();
        self.shape.update_parents(&mut self.clone());
    }

    pub fn transformed_ray(&self, ray: &Ray) -> Ray {
        ray.transform(&self.inverse_transform)
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let transformed_ray = self.transformed_ray(ray);
        self.shape.local_intersect(&transformed_ray, self)
    }

    pub fn normal_at(&self, world_point: &Point, hit: Option<Intersection>) -> Vector {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.local_normal_at(&local_point, hit);
        self.normal_to_world(&local_normal)
    }

    pub fn world_to_object(&self, p: &Point) -> Point {
        let point = if let Some(parent) = &self.parent {
            parent.world_to_object(p)
        } else {
            *p
        };
        self.inverse_transform * point
    }

    pub fn normal_to_world(&self, normal: &Vector) -> Vector {
        let mut normal = self.inverse_transform.transpose() * normal;
        normal.w = 0.0;
        if let Some(parent) = &self.parent {
            parent.normal_to_world(&normal.normalize())
        } else {
            normal.normalize()
        }
    }

    pub fn add_child(&mut self, child: &mut Object) {
        child.parent = Some(Arc::new(self.to_owned()));
        self.shape.add_child(child);
    }

    pub fn is_transparent(&self) -> bool {
        self.material.transparency == 0.0
    }

    pub fn is_reflective(&self) -> bool {
        self.material.reflective > 0.0
    }

    pub fn lighting(
        &self,
        light: &Light,
        point: &Point,
        eyev: &Vector,
        normalv: &Vector,
        in_shadow: bool,
    ) -> Color {
        self.material
            .lighting(self, light, point, eyev, normalv, in_shadow)
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.shape.bounding_box()
    }

    pub fn bounding_box_in_parent_space(&self) -> BoundingBox {
        self.bounding_box().transform(self.transform)
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::empty().unwrap()
    }
}

impl PartialEq for Object {
    // Ignore parent when comparing
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.material == other.material
            && self.shape == other.shape
    }
}

#[cfg(test)]
mod test_chapter_9_shapes {
    #![allow(non_snake_case)]

    use super::*;
    use crate::test_common::*;
    use std::f64::consts::PI;

    #[test]
    fn the_default_transformation() {
        let s = test_shape();
        assert_eq!(s.transform, identity_matrix());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut s = test_shape();
        let t = translation(2., 3., 4.);
        s.set_transform(t).unwrap();
        assert_eq!(s.transform, t);
    }

    #[test]
    fn the_default_material() {
        let s = test_shape();
        let m = s.material;
        assert_eq!(m, material());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = test_shape();
        let mut m = material();
        m.ambient = 1.;
        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(scaling(2.0, 2.0, 2.0)).unwrap();
        s.intersect(&r);
        let transformed_ray = s.transformed_ray(&r);
        assert_eq!(transformed_ray.origin, point(0.0, 0.0, -2.5));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(translation(5.0, 0.0, 0.0)).unwrap();
        s.intersect(&r);
        let transformed_ray = s.transformed_ray(&r);
        assert_eq!(transformed_ray.origin, point(-5.0, 0.0, -5.0));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = test_shape();
        s.set_transform(translation(0.0, 1.0, 0.0)).unwrap();
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711), None);
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = test_shape();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(m).unwrap();
        let n = s.normal_at(
            &point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
            None,
        );
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}

#[cfg(test)]
mod test_chapter_14_object_bounds {
    use super::*;
    use crate::test_common::*;

    #[test]
    fn objects_have_a_bounding_box_field_set_to_the_default_bounding_box() {
        let s = test_shape();
        assert_eq!(
            s.bounding_box(),
            bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0))
        );
    }

    #[test]
    fn objects_with_a_bounding_box_can_be_compared() {
        let s1 = test_shape();
        let s2 = test_shape();
        assert!(s1 == s2);
        assert!(s1.bounding_box() == s2.bounding_box());
        assert_eq!(s1.bounding_box(), s2.bounding_box());
    }

    #[test]
    fn querying_a_shapes_bounding_box_in_its_parents_space() {
        let mut shape = sphere().unwrap();
        shape
            .set_transform(translation(1.0, -3.0, 5.0) * scaling(0.5, 2.0, 4.0))
            .unwrap();
        let b = shape.bounding_box_in_parent_space();
        assert_eq!(b.minimum, point(0.5, -5.0, 1.0));
        assert_eq!(b.maximum, point(1.5, -1.0, 9.0));
    }
}
