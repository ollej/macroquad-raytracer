use crate::{intersection::*, material::*, matrix::*, ray::*, shape::*, sphere::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Object {
    pub transform: Matrix,
    pub material: Material,
    pub shape: Shape,
}

impl Object {
    pub fn empty() -> Self {
        Self {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            shape: Shape::Sphere(Sphere {}),
        }
    }

    pub fn new(matrix: Matrix) -> Self {
        Self {
            transform: matrix,
            material: Material::default(),
            shape: Shape::Sphere(Sphere {}),
        }
    }

    pub fn new_sphere(transform: Matrix, material: Material) -> Self {
        Self {
            transform,
            material,
            shape: Shape::Sphere(Sphere {}),
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
        if let Some((t1, t2)) = self.shape.intersect(&self.transformed_ray(ray)?) {
            Ok(Intersections::new(vec![
                Intersection::new(t1, self.to_owned()),
                Intersection::new(t2, self.to_owned()),
            ]))
        } else {
            Ok(Intersections::empty())
        }
    }

    pub fn normal_at(&self, p: &Point) -> Result<Vector, String> {
        let transform_inverse = self.transform.inverse()?;
        let object_point = transform_inverse * p;
        let object_normal = object_point - point(0., 0., 0.);
        let mut world_normal = transform_inverse.transpose() * object_normal;
        world_normal.w = 0.;
        Ok(world_normal.normalize())
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
            shape: Shape::Sphere(Sphere {}),
        }
    }
}

#[cfg(test)]
mod test_chapter_9_shapes {
    #![allow(non_snake_case)]

    use super::*;

    fn test_shape() -> Object {
        Object::empty()
    }

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
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(&scaling(2.0, 2.0, 2.0));
        s.intersect(&r).unwrap();
        let transformed_ray = s.transformed_ray(&r).unwrap();
        assert_eq!(transformed_ray.origin, point(0.0, 0.0, -2.5));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 0.5));
    }

    fn intersecting_a_translated_shape_with_a_ray() {
        let r = ray(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(&translation(5.0, 0.0, 0.0));
        s.intersect(&r).unwrap();
        let transformed_ray = s.transformed_ray(&r).unwrap();
        assert_eq!(transformed_ray.origin, point(-5.0, 0.0, -5.0));
        assert_eq!(transformed_ray.direction, vector(0.0, 0.0, 1.0));
    }
}
