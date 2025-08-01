use crate::{bounds::*, intersection::*, material::*, matrix::*, object::*, ray::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Cube {}

impl Cube {
    pub fn new() -> Self {
        Self {}
    }

    pub fn local_intersect(&self, ray: &Ray, object: &Object) -> Intersections {
        let (xtmin, xtmax) = self.check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (ytmin, ytmax) = self.check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (ztmin, ztmax) = self.check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);
        let tmin = xtmin.max(ytmin.max(ztmin));
        let tmax = xtmax.min(ytmax.min(ztmax));

        if tmax < 0.0 || tmin > tmax {
            Intersections::empty()
        } else {
            let xs = vec![tmin, tmax];
            Intersections::from_object(xs, object)
        }
    }

    pub fn local_normal_at(&self, point: &Point, _hit: Option<Intersection>) -> Vector {
        let maxc = point.x.abs().max(point.y.abs().max(point.z.abs()));
        if maxc == point.x.abs() {
            vector(point.x, 0.0, 0.0)
        } else if maxc == point.y.abs() {
            vector(0.0, point.y, 0.0)
        } else {
            vector(0.0, 0.0, point.z)
        }
    }
}

impl Axis for Cube {}
impl Bounds for Cube {}

pub fn cube() -> Object {
    Object::new_cube(IDENTITY_MATRIX, Material::default())
}

#[cfg(test)]
mod test_chapter_12_cube {
    use super::*;

    use crate::float::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = cube();

        let examples = [
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

        for (_name, origin, direction, t1, t2) in examples.iter() {
            let r = ray(&origin, &direction);
            let xs = c.intersect(&r).unwrap();
            assert_eq!(xs.len(), 2);
            assert_eq_float!(xs[0].t, t1);
            assert_eq_float!(xs[1].t, t2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let examples = [
            (point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
            (point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
            (point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
            (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
            (point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
            (point(0.0, 0.0, 2.0), vector(0.0, 0.0, 1.0)),
        ];

        let c = cube();
        for (origin, direction) in examples.iter() {
            let r = ray(&origin, &direction);
            let xs = c.intersect(&r).unwrap();
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = cube();
        let examples = [
            (point(1.0, 0.5, -0.8), vector(1.0, 0.0, 0.0)),
            (point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0)),
            (point(-0.4, 1.0, -0.1), vector(0.0, 1.0, 0.0)),
            (point(0.3, -1.0, -0.7), vector(0.0, -1.0, 0.0)),
            (point(-0.6, 0.3, 1.0), vector(0.0, 0.0, 1.0)),
            (point(0.4, 0.4, -1.0), vector(0.0, 0.0, -1.0)),
            (point(1.0, 1.0, 1.0), vector(1.0, 0.0, 0.0)),
            (point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0)),
        ];

        for (point, expected_normal) in examples.iter() {
            let p = point;
            let actual_normal = c.normal_at(&p, None).unwrap();
            assert_eq!(actual_normal, *expected_normal);
        }
    }
}

#[cfg(test)]
mod test_chapter_14_cubes_bounds {
    use super::*;

    #[test]
    fn cubes_have_a_default_bounding_box() {
        let s = cube();
        let b = s.bounding_box();
        assert_eq!(b.minimum, point(-1.0, -1.0, -1.0));
        assert_eq!(b.maximum, point(1.0, 1.0, 1.0));
    }
}
