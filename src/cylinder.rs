use crate::{
    bounds::*, float::*, intersection::*, material::*, matrix::*, object::*, ray::*, tuple::*,
};

use core::f64;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cylinder {
    minimum: Float,
    maximum: Float,
    closed: bool,
}

impl Cylinder {
    pub fn new(minimum: Float, maximum: Float, closed: bool) -> Self {
        Self {
            minimum,
            maximum,
            closed,
        }
    }

    pub fn infinite() -> Self {
        Self {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        let a = ray.direction.x.powf(2.0) + ray.direction.z.powf(2.0);

        let mut xs = vec![];

        // Ray is parallel to the y axis
        if a != 0.0 {
            let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
            let c = ray.origin.x.powf(2.0) + ray.origin.z.powf(2.0) - 1.0;
            xs.append(&mut self.intersect_walls(a, b, c, ray));
        }

        // Intersect with end caps
        if self.closed {
            xs.append(&mut self.intersect_caps(ray));
        }

        Intersections::from_object(xs, object)
    }

    pub fn local_normal_at(&self, p: &Point) -> Vector {
        // Compute the square of the distance from the y axis
        let distance = p.x.powf(2.0) + p.z.powf(2.0);

        if distance < 1.0 && p.y >= (self.maximum - EPSILON) {
            vector(0.0, 1.0, 0.0)
        } else if distance < 1.0 && p.y <= (self.minimum + EPSILON) {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(p.x, 0.0, p.z)
        }
    }
}

impl CylinderIntersection for Cylinder {
    fn minimum(&self) -> Float {
        self.minimum
    }

    fn maximum(&self) -> Float {
        self.maximum
    }

    fn radius(&self, _plane: Float) -> Float {
        1.0
    }
}

impl Bounds for Cylinder {
    fn bounding_box(&self) -> BoundingBox {
        BoundingBox {
            minimum: point(-1.0, self.minimum, -1.0),
            maximum: point(1.0, self.maximum, 1.0),
        }
    }
}

pub fn cylinder(minimum: Float, maximum: Float, closed: bool) -> Object {
    Object::new_cylinder(
        minimum,
        maximum,
        closed,
        IDENTITY_MATRIX,
        Material::default(),
    )
}

pub fn infinite_cylinder(translation: Matrix, material: Material) -> Object {
    Object::new_cylinder(
        f64::NEG_INFINITY,
        f64::INFINITY,
        false,
        translation,
        material,
    )
}

#[cfg(test)]
mod test_chapter_13_cylinder {
    use super::*;

    fn test_cylinder() -> Object {
        infinite_cylinder(IDENTITY_MATRIX, Material::default())
    }

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = test_cylinder();

        let examples = [
            (point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
        ];

        for (origin, direction) in examples.iter() {
            let normalized_direction = direction.normalize();
            let r = ray(&origin, &normalized_direction);
            let xs = cyl.intersect(&r).unwrap();
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let cyl = test_cylinder();

        let examples = [
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
            let xs = cyl.intersect(&r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0].t, t0);
            assert_eq_float!(xs[1].t, t1);
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let cyl = Cylinder::infinite();

        let examples = [
            (point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0)),
            (point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0)),
            (point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0)),
        ];

        for (point, normal) in examples.iter() {
            let n = cyl.local_normal_at(point);
            assert_eq!(n, *normal);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::infinite();
        assert_eq!(cyl.minimum, f64::NEG_INFINITY);
        assert_eq!(cyl.maximum, f64::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let cyl = cylinder(1.0, 2.0, false);

        let examples = [
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

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::infinite();
        assert_eq!(cyl.closed, false);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let cyl = cylinder(1.0, 2.0, true);

        let examples = [
            (point(0.0, 3.0, 0.0), vector(0.0, -1.0, 0.0), 2),
            (point(0.0, 3.0, -2.0), vector(0.0, -1.0, 2.0), 2),
            (point(0.0, 4.0, -2.0), vector(0.0, -1.0, 1.0), 2), // corner case
            (point(0.0, 0.0, -2.0), vector(0.0, 1.0, 2.0), 2),
            (point(0.0, -1.0, -2.0), vector(0.0, 1.0, 1.0), 2), // corner case
        ];

        for (p, direction, count) in examples.iter() {
            let direction = direction.normalize();
            let r = ray(&p, &direction);
            let xs = cyl.intersect(&r).unwrap();
            assert_eq!(xs.len(), *count);
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let cyl = Cylinder::new(1.0, 2.0, true);

        let examples = [
            (point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0)),
            (point(0.5, 1.0, 0.0), vector(0.0, -1.0, 0.0)),
            (point(0.0, 1.0, 0.5), vector(0.0, -1.0, 0.0)),
            (point(0.0, 2.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.5, 2.0, 0.0), vector(0.0, 1.0, 0.0)),
            (point(0.0, 2.0, 0.5), vector(0.0, 1.0, 0.0)),
        ];

        for (p, normal) in examples.iter() {
            let n = cyl.local_normal_at(p);
            assert_eq!(n, *normal);
        }
    }
}

#[cfg(test)]
mod test_chapter_14_cylinder_bounds {
    use super::*;

    #[test]
    fn an_unbounded_cylinder_has_a_bounding_box() {
        let cyl = infinite_cylinder(IDENTITY_MATRIX, Material::default());
        let b = cyl.bounding_box;
        assert_eq!(b.minimum, point(-1.0, f64::NEG_INFINITY, -1.0));
        assert_eq!(b.maximum, point(1.0, f64::INFINITY, 1.0));
    }

    #[test]
    fn cylinder_have_a_bounding_box_matching_minimum_and_maximum() {
        let cyl = cylinder(-5.0, 3.0, true);
        let b = cyl.bounding_box;
        assert_eq!(b.minimum, point(-1.0, -5.0, -1.0));
        assert_eq!(b.maximum, point(1.0, 3.0, 1.0));
    }
}
