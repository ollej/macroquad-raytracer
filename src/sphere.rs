use crate::{intersection::*, matrix::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Sphere {
    pub transform: Matrix,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn set_transform(&mut self, matrix: Matrix) {
        self.transform = matrix;
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let sphere_to_ray = ray.origin - point(0., 0., 0.);
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.;
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return Intersections::empty();
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        Intersections::new(vec![
            Intersection::new(t1, self.clone()),
            Intersection::new(t2, self.clone()),
        ])
    }
}

pub fn sphere() -> Sphere {
    Sphere::new()
}

pub fn intersect(sphere: &Sphere, ray: &Ray) -> Intersections {
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
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 4.0);
        assert_eq_float!(xs[1].t, 6.0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, 5.0);
        assert_eq_float!(xs[1].t, 5.0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = ray(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = ray(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, -1.0);
        assert_eq_float!(xs[1].t, 1.0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0].t, -6.0);
        assert_eq_float!(xs[1].t, -4.0);
        assert_eq!(s.intersect(&r), xs);
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
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }
}
