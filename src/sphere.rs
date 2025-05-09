use crate::{intersection::*, matrix::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn empty() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn new(matrix: Matrix) -> Self {
        Sphere { transform: matrix }
    }

    pub fn set_transform(&mut self, matrix: &Matrix) {
        self.transform = matrix.clone();
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
            Intersection::new(t1, self.clone()),
            Intersection::new(t2, self.clone()),
        ]))
    }
}

pub fn sphere() -> Sphere {
    Sphere::empty()
}

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Result<Intersections, String> {
    sphere.intersect(ray)
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
