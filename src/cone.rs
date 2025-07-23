use crate::{float::*, material::*, matrix::*, object::*, ray::*, tuple::*};
use std::mem;

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
            minimum: -f64::INFINITY,
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

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Float> {
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

        xs
    }

    fn intersect_walls(&self, a: Float, b: Float, c: Float, ray: &Ray) -> Vec<Float> {
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

    fn intersect_caps(&self, ray: &Ray) -> Vec<Float> {
        let mut xs = vec![];

        // Caps only matter if the cylinder might possibly be
        // intersected by the ray.
        if ray.direction.y == 0.0 {
            return vec![];
        }

        // Check for an intersection with the lower end cap by intersecting
        // the ray with the plane at y=cyl.minimum
        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, &t, self.minimum.abs()) {
            xs.push(t);
        }

        // Check for an intersection with the upper end cap by intersecting
        // the ray with the plane at y=cyl.maximum
        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if self.check_cap(ray, &t, self.maximum.abs()) {
            xs.push(t);
        }

        xs
    }

    // A helper function to reduce duplication.
    // checks to see if the intersection at `t` is within a radius
    // from the y axis.
    fn check_cap(&self, ray: &Ray, t: &Float, radius: Float) -> bool {
        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        (x.powf(2.0) + z.powf(2.0)) <= radius
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
    Object::new_cone(-f64::INFINITY, f64::INFINITY, false, translation, material)
}

#[cfg(test)]
mod test_chapter_13_cone {
    use super::*;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let shape = Cone::infinite();

        let examples = vec![
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
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0], t0);
            assert_eq_float!(xs[1], t1);
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::infinite();
        let direction = vector(0.0, 1.0, 1.0).normalize();
        let r = ray(&point(0.0, 0.0, -1.0), &direction);
        let xs = shape.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq_float!(xs[0], 0.35355);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let shape = Cone::new(-0.5, 0.5, true);

        let examples = vec![
            (point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4),
        ];

        for (origin, direction, count) in examples.iter() {
            let direction = direction.normalize();
            let r = ray(&origin, &direction);
            let xs = shape.local_intersect(&r);
            assert_eq!(xs.len(), *count);
        }
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let shape = Cone::infinite();

        let examples = vec![
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
