use crate::{matrix::*, tuple::*};

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
    fn new(minimum: &Point, maximum: &Point) -> BoundingBox {
        BoundingBox {
            minimum: minimum.to_owned(),
            maximum: maximum.to_owned(),
        }
    }

    fn empty() -> BoundingBox {
        BoundingBox {
            minimum: Point::empty_point(),
            maximum: Point::empty_point(),
        }
    }
}

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
        BoundingBox {
            minimum: self.minimum * rhs,
            maximum: self.maximum * rhs,
        }
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

impl Add<BoundingBox> for BoundingBox {
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
                f64::min(self.minimum.x, rhs.x),
                f64::min(self.minimum.y, rhs.y),
                f64::min(self.minimum.z, rhs.z),
            ),
            maximum: Point::point(
                f64::max(self.maximum.x, rhs.x),
                f64::max(self.maximum.y, rhs.y),
                f64::max(self.maximum.z, rhs.z),
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
    bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0))
}

#[cfg(test)]
mod test_chapter_14_bounds {
    use core::f64;

    use super::*;

    #[test]
    fn bounds_have_a_minimum_and_a_maximum_point() {
        let b = bounding_box(&point(-1.0, -1.0, -1.0), &point(1.0, 1.0, 1.0));
        assert_eq!(b.minimum, point(-1.0, -1.0, -1.0));
        assert_eq!(b.maximum, point(1.0, 1.0, 1.0));
    }

    #[test]
    fn bounds_can_be_compared() {
        let b = default_bounding_box();
        assert_eq!(b, default_bounding_box());
    }

    #[test]
    fn infinite_bounds_can_be_compared() {
        let b1 = bounding_box(
            &point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
            &point(f64::INFINITY, f64::INFINITY, f64::INFINITY),
        );
        let b2 = b1.clone();
        assert_eq!(b1, b2);
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
    fn multiply_bounding_boxes() {
        let b = default_bounding_box();
        let m = translation(1.0, -1.0, 1.0);
        assert_eq!(
            b * m,
            bounding_box(&point(0.0, -2.0, 0.0), &point(2.0, 0.0, 2.0))
        );
    }
}
