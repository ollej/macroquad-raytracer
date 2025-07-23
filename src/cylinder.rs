use {core::f64, std::mem};

use crate::{float::*, material::*, matrix::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cylinder {
    minimum: Float,
    maximum: Float,
}

impl Cylinder {
    pub fn new(minimum: Float, maximum: Float) -> Self {
        Self { minimum, maximum }
    }

    pub fn infinite() -> Self {
        Self {
            minimum: -f64::INFINITY,
            maximum: f64::INFINITY,
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Float> {
        let a = ray.direction.x.powf(2.0) + ray.direction.z.powf(2.0);

        // Ray is parallel to the y axis
        if a == 0.0 {
            return vec![];
        }

        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powf(2.0) + ray.origin.z.powf(2.0) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        // Ray does not intersect the cylinder
        if discriminant < 0.0 {
            return vec![];
        }

        // Ray intersects with the cylinder
        let t0 = &mut ((-b - f64::sqrt(discriminant)) / (2.0 * a));
        let t1 = &mut ((-b + f64::sqrt(discriminant)) / (2.0 * a));
        if t0 > t1 {
            mem::swap(t0, t1);
        }

        let mut xs = vec![];
        let y0 = ray.origin.y + *t0 * ray.direction.y;
        if self.minimum < y0 && y0 < self.maximum {
            xs.push(*t0);
        }
        let y1 = ray.origin.y + *t1 * ray.direction.y;
        if self.minimum < y1 && y1 < self.maximum {
            xs.push(*t1);
        }

        xs
    }

    pub fn normal_at(&self, p: &Point) -> Vector {
        vector(p.x, 0.0, p.z)
    }
}

pub fn cylinder(minimum: Float, maximum: Float) -> Object {
    Object::new_cylinder(minimum, maximum, IDENTITY_MATRIX, Material::default())
}

pub fn infinite_cylinder(translation: Matrix, material: Material) -> Object {
    Object::new_cylinder(-f64::INFINITY, f64::INFINITY, translation, material)
}

#[cfg(test)]
mod test_chapter_13_cylinder {
    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = Cylinder::infinite();

        let examples = vec![
            (point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
        ];

        for (origin, direction) in examples.iter() {
            let normalized_direction = direction.normalize();
            let r = ray(&origin, &normalized_direction);
            let xs = cyl.local_intersect(&r);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = Cylinder::infinite();

        let examples = vec![
            (point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0),
            (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
            (
                point(0.5, 0.0, -5.0),
                vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        for (origin, direction, t0, t1) in examples.iter() {
            let direction = direction.normalize();
            let r = ray(&origin, &direction);
            let xs = cyl.local_intersect(&r);

            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0], t0);
            assert_eq_float!(xs[1], t1);
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::infinite();

        let examples = vec![
            (point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0)),
            (point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0)),
            (point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0)),
        ];

        for (point, normal) in examples.iter() {
            let n = cyl.normal_at(point);
            assert_eq!(n, *normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::infinite();
        assert_eq!(cyl.minimum, -f64::INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = cylinder(1.0, 2.0);

        let examples: Vec<(Point, Vector, usize)> = vec![
            (point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0), 0),
            (point(0.0, 3.0, -5.0), vector(0.0, 0.0, 1.0), 0),
            (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 0),
            (point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0), 0),
            (point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0), 0),
            (point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0), 2),
        ];

        for (p, direction, count) in examples.iter() {
            let direction = direction.normalize();
            let r = ray(&p, &direction);
            let xs = cyl.intersect(&r).unwrap();
            assert_eq!(xs.len(), *count);
        }
    }
}
