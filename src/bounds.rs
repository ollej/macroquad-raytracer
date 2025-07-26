use crate::{intersection::*, matrix::*, ray::*, tuple::*};

use std::{
    iter::{Iterator, Sum},
    ops::{Add, Mul},
};

#[derive(PartialEq, Clone, Debug)]
pub struct BoundingBox {
    pub minimum: Point,
    pub maximum: Point,
}

impl BoundingBox {
    pub fn new(minimum: &Point, maximum: &Point) -> BoundingBox {
        BoundingBox {
            minimum: minimum.to_owned(),
            maximum: maximum.to_owned(),
        }
    }

    pub fn empty() -> BoundingBox {
        BoundingBox {
            minimum: Point::infinity_point(),
            maximum: Point::neg_infinity_point(),
        }
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        p.x >= self.minimum.x
            && p.x <= self.maximum.x
            && p.y >= self.minimum.y
            && p.y <= self.maximum.y
            && p.z >= self.minimum.z
            && p.z <= self.maximum.z
    }

    pub fn contains_bounding_box(&self, b: &BoundingBox) -> bool {
        self.contains_point(&b.minimum) && self.contains_point(&b.maximum)
    }

    pub fn transform(&self, matrix: Matrix) -> BoundingBox {
        let points = [
            self.minimum,
            point(self.minimum.x, self.minimum.y, self.maximum.z),
            point(self.minimum.x, self.maximum.y, self.minimum.z),
            point(self.minimum.x, self.maximum.y, self.maximum.z),
            point(self.maximum.x, self.minimum.y, self.minimum.z),
            point(self.maximum.x, self.minimum.y, self.maximum.z),
            point(self.maximum.x, self.maximum.y, self.minimum.z),
            self.maximum,
        ];

        let mut new_bbox = BoundingBox::empty();

        for p in points.iter() {
            new_bbox = new_bbox + matrix * p;
        }

        new_bbox
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) = self.check_axis(
            ray.origin.x,
            ray.direction.x,
            self.minimum.x,
            self.maximum.x,
        );
        let (ytmin, ytmax) = self.check_axis(
            ray.origin.y,
            ray.direction.y,
            self.minimum.y,
            self.maximum.y,
        );
        let (ztmin, ztmax) = self.check_axis(
            ray.origin.z,
            ray.direction.z,
            self.minimum.z,
            self.maximum.z,
        );

        let tmax = xtmax.min(ytmax.min(ztmax));
        if tmax < 0.0 {
            false
        } else {
            let tmin = xtmin.max(ytmin.max(ztmin));
            tmin <= tmax
        }
    }
}

impl Axis for BoundingBox {}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::empty()
    }
}

pub trait Bounds {
    fn bounding_box(&self) -> BoundingBox {
        default_bounding_box()
    }
}

impl Mul<Matrix> for BoundingBox {
    type Output = BoundingBox;

    fn mul(self, rhs: Matrix) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for &BoundingBox {
    type Output = BoundingBox;

    fn mul(self, rhs: Matrix) -> Self::Output {
        BoundingBox {
            minimum: self.minimum * rhs,
            maximum: self.maximum * rhs,
        }
    }
}

impl Add for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: BoundingBox) -> Self::Output {
        self + rhs.minimum + rhs.maximum
    }
}

impl Add<Point> for BoundingBox {
    type Output = BoundingBox;

    fn add(self, rhs: Point) -> Self::Output {
        BoundingBox {
            minimum: Point::point(
                self.minimum.x.min(rhs.x),
                self.minimum.y.min(rhs.y),
                self.minimum.z.min(rhs.z),
            ),
            maximum: Point::point(
                self.maximum.x.max(rhs.x),
                self.maximum.y.max(rhs.y),
                self.maximum.z.max(rhs.z),
            ),
        }
    }
}

impl Sum for BoundingBox {
    fn sum<T: Iterator<Item = BoundingBox>>(iter: T) -> BoundingBox {
        iter.fold(BoundingBox::empty(), |a, b| a + b)
    }
}

pub fn bounding_box(minimum: &Point, maximum: &Point) -> BoundingBox {
    BoundingBox::new(minimum, maximum)
}

fn default_bounding_box() -> BoundingBox {
    BoundingBox::new(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0))
}

fn empty_bounding_box() -> BoundingBox {
    BoundingBox::empty()
}

#[cfg(test)]
mod test_chapter_14_bounds {
    use super::*;

    use std::f64::consts::PI;

    #[test]
    fn an_empty_bounding_box_has_infinite_min_and_max() {
        let b = empty_bounding_box();
        assert_eq!(
            b.minimum,
            point(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );
        assert_eq!(
            b.maximum,
            point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        );
    }

    #[test]
    fn a_bounding_box_with_volume() {
        let b = bounding_box(&point(-1.0, -2.0, -3.0), &point(1.0, 1.0, 1.0));
        assert_eq!(b.minimum, point(-1.0, -2.0, -3.0));
        assert_eq!(b.maximum, point(1.0, 1.0, 1.0));
    }

    #[test]
    fn bounds_can_be_compared() {
        let b = default_bounding_box();
        assert_eq!(b, default_bounding_box());
    }

    #[test]
    fn infinite_bounds_can_be_compared() {
        let b1 = empty_bounding_box();
        let b2 = empty_bounding_box();
        assert_eq!(b1, b2);
    }

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let mut b = empty_bounding_box();
        let p1 = point(-5.0, 2.0, 0.0);
        let p2 = point(7.0, 0.0, -3.0);
        b = b + p1;
        b = b + p2;
        assert_eq!(b.minimum, point(-5.0, 0.0, -3.0));
        assert_eq!(b.maximum, point(7.0, 2.0, 0.0));
    }

    #[test]
    fn bounds_can_be_added() {
        let b1 = default_bounding_box();
        let b2 = bounding_box(&point(0.0, -2.0, -3.0), &point(0.0, 1.0, 2.0));
        assert_eq!(
            b1 + b2,
            bounding_box(&point(-1.0, -2.0, -3.0), &point(1.0, 1.0, 2.0))
        );
    }

    #[test]
    fn add_point_to_bounding_box() {
        let b = default_bounding_box();
        let p = point(5.0, -5.0, 0.0);
        assert_eq!(
            b + p,
            bounding_box(&point(-1.0, -5.0, -1.0), &point(5.0, 1.0, 1.0))
        );
    }

    #[test]
    fn sum_bounding_boxes() {
        let b1 = default_bounding_box();
        let b2 = bounding_box(&point(0.0, -2.0, -3.0), &point(0.0, 1.0, 2.0));
        let bbs: Vec<BoundingBox> = vec![b1, b2];
        assert_eq!(
            bbs.into_iter().sum::<BoundingBox>(),
            bounding_box(&point(-1.0, -2.0, -3.0), &point(1.0, 1.0, 2.0))
        );
    }

    #[test]
    fn adding_one_bounding_box_to_another() {
        let box1 = bounding_box(&point(-5.0, -2.0, 0.0), &point(7.0, 4.0, 4.0));
        let box2 = bounding_box(&point(8.0, -7.0, -2.0), &point(14.0, 2.0, 8.0));
        let b = box1 + box2;
        assert_eq!(b.minimum, point(-5.0, -7.0, -2.0));
        assert_eq!(b.maximum, point(14.0, 4.0, 8.0));
    }

    #[test]
    fn multiply_bounding_boxes() {
        let b = default_bounding_box();
        let m = translation(1.0, -1.0, 1.0);
        assert_eq!(
            b * m,
            bounding_box(&point(0.0, -2.0, 0.0), &point(2.0, 0.0, 2.0))
        );
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_point() {
        let b = bounding_box(&point(5.0, -2.0, 0.0), &point(11.0, 4.0, 7.0));

        let examples = [
            (point(5.0, -2.0, 0.0), true),
            (point(11.0, 4.0, 7.0), true),
            (point(8.0, 1.0, 3.0), true),
            (point(3.0, 0.0, 3.0), false),
            (point(8.0, -4.0, 3.0), false),
            (point(8.0, 1.0, -1.0), false),
            (point(13.0, 1.0, 3.0), false),
            (point(8.0, 5.0, 3.0), false),
            (point(8.0, 1.0, 8.0), false),
        ];

        for (p, result) in examples.iter() {
            assert_eq!(b.contains_point(&p), *result);
        }
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_box() {
        let b = bounding_box(&point(5.0, -2.0, 0.0), &point(11.0, 4.0, 7.0));

        let examples = [
            (point(5.0, -2.0, 0.0), point(11.0, 4.0, 7.0), true),
            (point(6.0, -1.0, 1.0), point(10.0, 3.0, 6.0), true),
            (point(4.0, -3.0, -1.0), point(10.0, 3.0, 6.0), false),
            (point(6.0, -1.0, 1.0), point(12.0, 5.0, 8.0), false),
        ];

        for (min, max, result) in examples.iter() {
            let b2 = bounding_box(&min, &max);
            assert_eq!(b.contains_bounding_box(&b2), *result);
        }
    }

    #[test]
    fn transforming_a_bounding_box() {
        let b = bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0));
        let matrix = rotation_x(PI / 4.0) * rotation_y(PI / 4.0);
        let b2 = b.transform(matrix);
        assert_eq!(b2.minimum, point(-1.4142, -1.7071, -1.7071));
        assert_eq!(b2.maximum, point(1.4142, 1.7071, 1.7071));
    }

    #[test]
    fn a_ray_intersects_a_bounding_box() {
        let b = default_bounding_box();

        let examples = [
            // ( name , origin , direction , t1 , t2 )
            ("+x", point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0)),
            ("-x", point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0)),
            ("+y", point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0)),
            ("-y", point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0)),
            ("+z", point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0)),
            ("-z", point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0)),
            ("inside", point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0)),
        ];

        for (_name, origin, direction) in examples.iter() {
            let r = ray(&origin, &direction);
            assert!(b.intersects(&r));
        }
    }

    #[test]
    fn intersecting_a_ray_with_a_bounding_box_at_the_origin() {
        let b = bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0));

        let examples = [
            (point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), true),
            (point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), true),
            (point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), true),
            (point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), true),
            (point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), true),
            (point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), true),
            (point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), true),
            (point(-2.0, 0.0, 0.0), vector(2.0, 4.0, 6.0), false),
            (point(0.0, -2.0, 0.0), vector(6.0, 2.0, 4.0), false),
            (point(0.0, 0.0, -2.0), vector(4.0, 6.0, 2.0), false),
            (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0), false),
            (point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0), false),
            (point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0), false),
        ];

        for (origin, direction, result) in examples.iter() {
            let normalized_direction = direction.normalize();
            let r = ray(&origin, &normalized_direction);
            assert_eq!(b.intersects(&r), *result);
        }
    }

    #[test]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let b = bounding_box(&point(5.0, -2.0, 0.0), &point(11.0, 4.0, 7.0));

        let examples = [
            (point(15.0, 1.0, 2.0), vector(-1.0, 0.0, 0.0), true),
            (point(-5.0, -1.0, 4.0), vector(1.0, 0.0, 0.0), true),
            (point(7.0, 6.0, 5.0), vector(0.0, -1.0, 0.0), true),
            (point(9.0, -5.0, 6.0), vector(0.0, 1.0, 0.0), true),
            (point(8.0, 2.0, 12.0), vector(0.0, 0.0, -1.0), true),
            (point(6.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), true),
            (point(8.0, 1.0, 3.5), vector(0.0, 0.0, 1.0), true),
            (point(9.0, -1.0, -8.0), vector(2.0, 4.0, 6.0), false),
            (point(8.0, 3.0, -4.0), vector(6.0, 2.0, 4.0), false),
            (point(9.0, -1.0, -2.0), vector(4.0, 6.0, 2.0), false),
            (point(4.0, 0.0, 9.0), vector(0.0, 0.0, -1.0), false),
            (point(8.0, 6.0, -1.0), vector(0.0, -1.0, 0.0), false),
            (point(12.0, 5.0, 4.0), vector(-1.0, 0.0, 0.0), false),
        ];

        for (origin, direction, result) in examples.iter() {
            let normalized_direction = direction.normalize();
            let r = ray(&origin, &normalized_direction);
            assert_eq!(b.intersects(&r), *result);
        }
    }

    #[test]
    fn a_ray_misses_a_bounding_box() {
        let b = default_bounding_box();

        let examples = [
            (point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
            (point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
            (point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
            (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
            (point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
            (point(0.0, 0.0, 2.0), vector(0.0, 0.0, 1.0)),
        ];

        for (origin, direction) in examples.iter() {
            let r = ray(&origin, &direction);
            assert_eq!(b.intersects(&r), false);
        }
    }
}
