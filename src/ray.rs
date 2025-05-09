use crate::{float::*, sphere::*, tuple::*};

#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: Float) -> Point {
        self.origin + self.direction * t
    }
}

pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray::new(origin, direction)
}

pub fn position(ray: &Ray, t: Float) -> Point {
    ray.position(t)
}

#[cfg(test)]
mod test_chapter_5_intersections {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = point(1., 2., 3.);
        let direction = vector(4., 5., 6.);
        let r = ray(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
        let r = Ray::new(origin, direction);
        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = ray(point(2., 3., 4.), vector(1., 0., 0.));
        assert_eq!(position(&r, 0.), point(2., 3., 4.));
        assert_eq!(position(&r, 1.), point(3., 3., 4.));
        assert_eq!(position(&r, -1.), point(1., 3., 4.));
        assert_eq!(position(&r, 2.5), point(4.5, 3., 4.));
        assert_eq!(r.position(0.), point(2., 3., 4.));
        assert_eq!(r.position(1.), point(3., 3., 4.));
        assert_eq!(r.position(-1.), point(1., 3., 4.));
        assert_eq!(r.position(2.5), point(4.5, 3., 4.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = ray(point(0., 0., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0], 4.0);
        assert_eq_float!(xs[1], 6.0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = ray(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0], 5.0);
        assert_eq_float!(xs[1], 5.0);
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
        assert_eq_float!(xs[0], -1.0);
        assert_eq_float!(xs[1], 1.0);
        assert_eq!(s.intersect(&r), xs);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = ray(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert_eq!(xs.len(), 2);
        assert_eq_float!(xs[0], -6.0);
        assert_eq_float!(xs[1], -4.0);
        assert_eq!(s.intersect(&r), xs);
    }
}
