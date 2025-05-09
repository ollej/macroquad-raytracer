use crate::{float::*, tuple::*};

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
mod test_chapter_4_transformations {
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
}
