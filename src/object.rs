use crate::{
    bounds::*, color::*, cone::*, cube::*, cylinder::*, float::*, group::*, intersection::*,
    light::*, material::*, matrix::*, plane::*, ray::*, shape::*, sphere::*, tuple::*,
};

use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Object {
    pub transform: Matrix,
    pub material: Material,
    pub bounding_box: BoundingBox,
    pub shape: Shape,
    pub parent: Option<Arc<Object>>,
}

impl Object {
    pub fn empty() -> Self {
        Self::new(IDENTITY_MATRIX)
    }

    pub fn new(transform: Matrix) -> Self {
        Self::new_sphere(transform, Material::default())
    }

    pub fn new_sphere(transform: Matrix, material: Material) -> Self {
        let shape = Shape::Sphere(Sphere {});
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn new_plane(transform: Matrix, material: Material) -> Self {
        let shape = Shape::Plane(Plane {});
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn new_cube(transform: Matrix, material: Material) -> Self {
        let shape = Shape::Cube(Cube {});
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn new_cylinder(
        minimum: Float,
        maximum: Float,
        closed: bool,
        transform: Matrix,
        material: Material,
    ) -> Self {
        let shape = Shape::Cylinder(Cylinder::new(minimum, maximum, closed));
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn new_cone(
        minimum: Float,
        maximum: Float,
        closed: bool,
        transform: Matrix,
        material: Material,
    ) -> Self {
        let shape = Shape::Cone(Cone::new(minimum, maximum, closed));
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn new_group(transform: Matrix, material: Material) -> Self {
        let shape = Shape::Group(Group::empty());
        Self {
            transform,
            material,
            bounding_box: shape.bounding_box(),
            shape,
            parent: None,
        }
    }

    pub fn set_transform(&mut self, matrix: &Matrix) {
        self.transform = matrix.to_owned();
    }

    pub fn set_material(&mut self, material: &Material) {
        self.material = material.to_owned();
    }

    pub fn transformed_ray(&self, ray: &Ray) -> Result<Ray, String> {
        Ok(ray.transform(&self.transform.inverse()?))
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Intersections, String> {
        let transformed_ray = self.transformed_ray(ray)?;
        self.shape.local_intersect(&transformed_ray, self)
    }

    pub fn normal_at(&self, world_point: &Point) -> Result<Vector, String> {
        let local_point = self.world_to_object(world_point)?;
        let local_normal = self.shape.local_normal_at(&local_point);
        self.normal_to_world(&local_normal)
    }

    pub fn world_to_object(&self, p: &Point) -> Result<Point, String> {
        let inverse_transform = self.transform.inverse()?;
        let point = if let Some(parent) = &self.parent {
            parent.world_to_object(p)?
        } else {
            *p
        };
        Ok(inverse_transform * point)
    }

    pub fn normal_to_world(&self, normal: &Vector) -> Result<Vector, String> {
        let mut normal = self.transform.inverse()?.transpose() * normal;
        normal.w = 0.0;
        let normalized_normal = normal.normalize();
        if let Some(parent) = &self.parent {
            parent.normal_to_world(&normalized_normal)
        } else {
            Ok(normalized_normal)
        }
    }

    pub fn add_child(&mut self, child: &mut Object) {
        child.parent = Some(Arc::new(self.to_owned()));
        self.shape.add_child(child);
        self.bounding_box = self.shape.bounding_box();
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
    ) -> Result<Color, String> {
        self.material
            .lighting(self, light, point, eyev, normalv, in_shadow)
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::empty()
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform
            && self.material == other.material
            && self.shape == other.shape
            && self.parent == other.parent
        //&& self.bounding_box == other.bounding_box
    }
}

#[cfg(test)]
mod test_common {
    use super::*;

    pub fn test_shape() -> Object {
        Object::empty()
    }
}

#[cfg(test)]
mod test_chapter_9_shapes {
    #![allow(non_snake_case)]

    use super::test_common::*;
    use super::*;
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
        s.set_transform(&t);
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
        s.set_transform(&scaling(2.0, 2.0, 2.0));
        s.intersect(&r).unwrap();
        let transformed_ray = s.transformed_ray(&r).unwrap();
        assert_eq!(transformed_ray.origin, point(0.0, 0.0, -2.5));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 0.5));
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(&translation(5.0, 0.0, 0.0));
        s.intersect(&r).unwrap();
        let transformed_ray = s.transformed_ray(&r).unwrap();
        assert_eq!(transformed_ray.origin, point(-5.0, 0.0, -5.0));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = test_shape();
        s.set_transform(&translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711)).unwrap();
        assert_eq!(n, vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = test_shape();
        let m = scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s
            .normal_at(&point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0))
            .unwrap();
        assert_eq!(n, vector(0.0, 0.97014, -0.24254));
    }
}

#[cfg(test)]
mod test_chapter_14_object_bounds {
    use super::test_common::*;
    use super::*;

    #[test]
    fn objects_have_a_bounding_box_field_set_to_the_default_bounding_box() {
        let s = test_shape();
        assert_eq!(
            s.bounding_box,
            bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0))
        );
    }
}
