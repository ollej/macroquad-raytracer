use crate::{
    bounds::*, float::*, intersection::*, material::*, matrix::*, object::*, ray::*, tuple::*,
};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cone {
    minimum: Float,
    maximum: Float,
    closed: bool,
}

impl Cone {
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

    pub fn local_normal_at(&self, p: &Point) -> Vector {
        // Compute the square of the distance from the y axis
        let distance = p.x.powf(2.0) + p.z.powf(2.0);

        if distance < 1.0 && p.y >= (self.maximum - EPSILON) {
            vector(0.0, 1.0, 0.0)
        } else if distance < 1.0 && p.y <= (self.minimum + EPSILON) {
            vector(0.0, -1.0, 0.0)
        } else {
            let mut y = f64::sqrt(p.x.powf(2.0) + p.z.powf(2.0));
            if p.y > 0.0 {
                y = -y;
            }

            vector(p.x, y, p.z)
        }
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        let a = ray.direction.x.powf(2.0) - ray.direction.y.powf(2.0) + ray.direction.z.powf(2.0);
        let b = 2.0 * ray.origin.x * ray.direction.x - 2.0 * ray.origin.y * ray.direction.y
            + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powf(2.0) - ray.origin.y.powf(2.0) + ray.origin.z.powf(2.0);

        let mut xs = vec![];

        if a == 0.0 {
            if b != 0.0 {
                let t = -(c / (2.0 * b));
                xs.push(t);
            }
        } else {
            xs.append(&mut self.intersect_walls(a, b, c, ray));
        }

        // Intersect with end caps
        if self.closed {
            xs.append(&mut self.intersect_caps(ray));
        }

        Intersections::from_object(xs, object)
    }
}

impl CylinderIntersection for Cone {
    fn minimum(&self) -> Float {
        self.minimum
    }

    fn maximum(&self) -> Float {
        self.maximum
    }

    fn radius(&self, plane: Float) -> Float {
        plane.abs()
    }
}

impl Bounds for Cone {
    fn bounding_box(&self) -> BoundingBox {
        let a = self.minimum.abs();
        let b = self.maximum.abs();
        let limit = a.max(b);

        BoundingBox {
            minimum: point(-limit, self.minimum, -limit),
            maximum: point(limit, self.maximum, limit),
        }
    }
}

pub fn cone(minimum: Float, maximum: Float, closed: bool) -> Object {
    Object::new_cone(
        minimum,
        maximum,
        closed,
        IDENTITY_MATRIX,
        Material::default(),
    )
}

pub fn infinite_cone(translation: Matrix, material: Material) -> Object {
    Object::new_cone(
        f64::NEG_INFINITY,
        f64::INFINITY,
        false,
        translation,
        material,
    )
}

pub fn unit_cone(closed: bool, transform: Matrix, material: Material) -> Object {
    Object::new_cone(
        -1.0,
        0.0,
        closed,
        translation(0.0, 1.0, 0.0) * transform,
        material,
    )
}

pub fn unit_cone_upsidedown(closed: bool, transform: Matrix, material: Material) -> Object {
    Object::new_cone(
        0.0,
        1.0,
        closed,
        translation(0.0, 0.0, 0.0) * transform,
        material,
    )
}

#[cfg(test)]
mod test_chapter_13_cone {
    use super::*;

    fn test_cone() -> Object {
        infinite_cone(IDENTITY_MATRIX, Material::default())
    }

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let shape = test_cone();

        let examples = [
            (point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0),
            (
                point(0.0, 0.0, -5.0),
                vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                point(1.0, 1.0, -5.0),
                vector(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];

        for (origin, direction, t0, t1) in examples.iter() {
            let normalized_direction = direction.normalize();
            let r = ray(&origin, &normalized_direction);
            let xs = shape.intersect(&r).unwrap();
            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0].t, t0);
            assert_eq_float!(xs[1].t, t1);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = test_cone();
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(&point(0.0, 0.0, -1.0), &direction);
        let xs = shape.intersect(&r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq_float!(xs[0].t, 0.35355);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let shape = cone(-0.5, 0.5, true);

        let examples = [
            (point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4),
        ];

        for (origin, direction, count) in examples.iter() {
            let direction = direction.normalize();
            let r = ray(&origin, &direction);
            let xs = shape.intersect(&r).unwrap();
            assert_eq!(xs.len(), *count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = Cone::infinite();

        let examples = [
            (point(0.0, 0.0, 0.0), vector(0.0, 0.0, 0.0)),
            (point(1.0, 1.0, 1.0), vector(1.0, -f64::sqrt(2.0), 1.0)),
            (point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0)),
        ];

        for (p, normal) in examples.iter() {
            let n = shape.local_normal_at(p);
            assert_eq!(n, *normal);
        }
    }
}

#[cfg(test)]
mod test_chapter_14_cone_bounds {
    use super::*;

    #[test]
    fn an_unbounded_cylinder_has_a_bounding_box() {
        let c = infinite_cone(IDENTITY_MATRIX, Material::default());
        let b = c.bounding_box();
        assert_eq!(
            b.minimum,
            point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        );
        assert_eq!(
            b.maximum,
            point(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );
    }

    #[test]
    fn cylinder_have_a_bounding_box_matching_minimum_and_maximum() {
        let c = cone(-5.0, 3.0, true);
        let b = c.bounding_box();
        assert_eq!(b.minimum, point(-5.0, -5.0, -5.0));
        assert_eq!(b.maximum, point(5.0, 3.0, 5.0));
    }
}
