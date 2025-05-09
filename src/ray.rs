use std::ops::Mul;

use crate::{float::*, matrix::*, tuple::*};

#[derive(PartialEq, Clone, Debug)]
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

    pub fn transform(&self, matrix: &Matrix) -> Ray {
        self * matrix
    }
}

impl Mul<Matrix> for &Ray {
    type Output = Ray;

    fn mul(self, other: Matrix) -> Self::Output {
        Ray {
            origin: self.origin * other,
            direction: self.direction * other,
        }
    }
}

impl Mul<&Matrix> for &Ray {
    type Output = Ray;

    fn mul(self, other: &Matrix) -> Self::Output {
        self * *other
    }
}

pub fn ray(origin: Point, direction: Vector) -> Ray {
    Ray::new(origin, direction)
}

pub fn position(ray: &Ray, t: Float) -> Point {
    ray.position(t)
}

pub fn transform(ray: &Ray, matrix: &Matrix) -> Ray {
    ray.transform(matrix)
}

#[cfg(test)]
mod test_chapter_5_ray {
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
    fn translating_a_ray() {
        let r = ray(point(1., 2., 3.), vector(0., 1., 0.));
        let m = translation(3., 4., 5.);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(4., 6., 8.));
        assert_eq!(r2.direction, vector(0., 1., 0.));
        assert_eq!(r.transform(&m), r2);
    }

    #[test]
    fn scaling_a_ray() {
        let r = ray(point(1., 2., 3.), vector(0., 1., 0.));
        let m = scaling(2., 3., 4.);
        let r2 = transform(&r, &m);
        assert_eq!(r2.origin, point(2., 6., 12.));
        assert_eq!(r2.direction, vector(0., 3., 0.));
        assert_eq!(r.transform(&m), r2);
    }
}
