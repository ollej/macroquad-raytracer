use crate::{intersection::*, material::*, matrix::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Sphere {
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn empty() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
            material: Material::default(),
        }
    }

    pub fn new(matrix: Matrix) -> Self {
        Sphere {
            transform: matrix,
            material: Material::default(),
        }
    }

    pub fn set_transform(&mut self, matrix: &Matrix) {
        self.transform = matrix.to_owned();
    }

    pub fn set_material(&mut self, material: &Material) {
        self.material = material.to_owned();
    }

    pub fn intersect(&self, ray: &Ray) -> Result<Intersections, String> {
        let ray2 = ray.transform(&self.transform.inverse()?);

        let sphere_to_ray = ray2.origin - point(0., 0., 0.);
        let a = ray2.direction.dot(&ray2.direction);
        let b = 2. * ray2.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Ok(Intersections::empty());
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Ok(Intersections::new(vec![
            Intersection::new(t1, self),
            Intersection::new(t2, self),
        ]))
    }

    pub fn normal_at(&self, p: &Point) -> Result<Vector, String> {
        let object_point = self.transform.inverse()? * p;
        let object_normal = object_point - point(0., 0., 0.);
        let mut world_normal = self.transform.inverse()?.transpose() * object_normal;
        world_normal.w = 0.;
        Ok(world_normal.normalize())
    }
}

pub fn sphere() -> Sphere {
    Sphere::empty()
}

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Result<Intersections, String> {
    sphere.intersect(ray)
}

pub fn normal_at(sphere: &Sphere, p: &Point) -> Result<Vector, String> {
    sphere.normal_at(p)
}

#[cfg(test)]
mod test_chapter_5_intersections {
    #![allow(non_snake_case)]

    use super::*;

    use crate::float::*;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 4.0);
        assert_eq_float!(xs[1].t, 6.0);
        assert_eq!(s.intersect(&r).unwrap(), xs);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 5.0);
        assert_eq_float!(xs[1].t, 5.0);
        assert_eq!(s.intersect(&r).unwrap(), xs);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(xs.len(), 0);
        assert_eq!(s.intersect(&r).unwrap(), xs);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, -1.0);
        assert_eq_float!(xs[1].t, 1.0);
        assert_eq!(s.intersect(&r).unwrap(), xs);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, -6.0);
        assert_eq_float!(xs[1].t, -4.0);
        assert_eq!(s.intersect(&r).unwrap(), xs);
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = sphere();
        assert_eq!(s.transform, identity_matrix());
    }

    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = sphere();
        let t = translation(2., 3., 4.);
        s.set_transform(&t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.set_transform(&scaling(2., 2., 2.));
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 3.0);
        assert_eq_float!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.set_transform(&translation(5., 0., 0.));
        let xs = s.intersect(&r).unwrap();
        assert_eq!(xs.len(), 0);
    }
}

#[cfg(test)]
mod test_chapter_6_normals {
    #![allow(non_snake_case)]

    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = sphere();
        let n = s.normal_at(&point(1., 0., 0.));
        assert_eq!(n, Ok(vector(1., 0., 0.)));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0., 1., 0.));
        assert_eq!(n, Ok(vector(0., 1., 0.)));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = sphere();
        let n = s.normal_at(&point(0., 0., 1.));
        assert_eq!(n, Ok(vector(0., 0., 1.)));
    }

    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = sphere();
        let n = s.normal_at(&point(
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
            3.0_f32.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Ok(vector(
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0
            ))
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = sphere();
        let n = s
            .normal_at(&point(
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
                3.0_f32.sqrt() / 3.0,
            ))
            .unwrap();
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = sphere();
        s.set_transform(&translation(0., 1., 0.));
        let n = s.normal_at(&point(0., 1.70711, -0.70711));
        assert_eq!(n, Ok(vector(0., 0.70711, -0.70711)));
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = sphere();
        let m = scaling(1., 0.5, 1.) * rotation_z(PI / 5.);
        s.set_transform(&m);
        let n = s.normal_at(&point(0., 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.));
        assert_eq!(n, Ok(vector(0., 0.97014, -0.24254)));
    }
}

#[cfg(test)]
mod test_chapter_6_sphere_material {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = sphere();
        let m = s.material;
        assert_eq!(m, material());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = sphere();
        let mut m = material();
        m.ambient = 1.;
        s.material = m;
        assert_eq!(s.material, m);
    }
}
