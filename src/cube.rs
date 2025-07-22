use crate::{
    float::*, intersection::*, material::*, matrix::*, object::*, ray::*, shape::*, tuple::*,
};

use std::mem;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cube {}

impl Cube {
    pub fn new() -> Self {
        Self {}
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Float> {
        let (xtmin, xtmax) = self.check_axis(&ray.origin.x, &ray.direction.x);
        let (ytmin, ytmax) = self.check_axis(&ray.origin.y, &ray.direction.y);
        let (ztmin, ztmax) = self.check_axis(&ray.origin.z, &ray.direction.z);
        let tmin = vec![xtmin, ytmin, ztmin]
            .iter()
            .max_by(|a: &&Float, b: &&Float| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let tmax = vec![xtmax, ytmax, ztmax]
            .iter()
            .min_by(|a: &&Float, b: &&Float| a.total_cmp(b))
            .unwrap()
            .to_owned();

        vec![tmin, tmax]
    }

    pub fn normal_at(&self, p: &Point) -> Point {
        // TODO
        p - &point(0., 0., 0.)
    }

    fn check_axis(&self, origin: &Float, direction: &Float) -> (Float, Float) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;
        let (tmin, tmax) = if direction.abs() >= EPSILON {
            (
                &mut (tmin_numerator / direction),
                &mut (tmax_numerator / direction),
            )
        } else {
            (
                &mut (tmin_numerator * f64::INFINITY),
                &mut (tmax_numerator * f64::INFINITY),
            )
        };
        if tmin > tmax {
            mem::swap(tmin, tmax);
        }
        (tmin.to_owned(), tmax.to_owned())
    }
}

pub fn cube() -> Object {
    Object::new_cube(IDENTITY_MATRIX, Material::default())
}

#[cfg(test)]
mod test_chapter_12_cube {
    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = cube();

        let examples = vec![
            // ( name , origin , direction , t1 , t2 )
            ("+x", point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
            ("-x", point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
            ("+y", point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
            ("-y", point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
            ("+z", point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
            ("-z", point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
            (
                "inside",
                point(0.0, 0.5, 0.0),
                vector(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        for example in examples.iter() {
            let r = ray(&example.1, &example.2);
            let xs = c.intersect(&r).unwrap();
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].t, example.3);
            assert_eq!(xs[1].t, example.4);
        }
    }
}
