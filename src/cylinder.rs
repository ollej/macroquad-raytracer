use crate::{float::*, material::*, matrix::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cylinder {}

impl Cylinder {
    pub fn new() -> Self {
        Self {}
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
        let t0 = (-b - f64::sqrt(discriminant)) / (2.0 * a);
        let t1 = (-b + f64::sqrt(discriminant)) / (2.0 * a);

        vec![t0, t1]
    }

    pub fn normal_at(&self, p: &Point) -> Vector {
        p - &point(0., 0., 0.)
    }
}

pub fn cylinder() -> Object {
    Object::new_cylinder(IDENTITY_MATRIX, Material::default())
}

#[cfg(test)]
mod test_chapter_13_cylinder {
    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let cyl = cylinder();

        let examples = vec![
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
        let cyl = cylinder();

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
            let xs = cyl.intersect(&r).unwrap();

            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0].t, t0);
            assert_eq_float!(xs[1].t, t1);
        }
    }
}
