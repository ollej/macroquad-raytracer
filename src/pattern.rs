use crate::{color::*, matrix::*, object::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pattern {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Pattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern {
            a,
            b,
            transform: IDENTITY_MATRIX,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn stripe_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_object(&self, object: &Object, point: &Point) -> Result<Color, String> {
        let object_point = object.transform.inverse()? * point;
        let pattern_point = self.transform.inverse()? * object_point;
        Ok(self.stripe_at(&pattern_point))
    }
}

pub fn stripe_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(a.to_owned(), b.to_owned())
}

pub fn stripe_at(pattern: &Pattern, point: &Point) -> Color {
    pattern.stripe_at(point)
}

#[cfg(test)]
mod test_chapter_10_pattern {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 2.0, 0.0)), WHITE);

        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(&WHITE, &BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = sphere();
        object.set_transform(&scaling(2.0, 2.0, 2.0));
        let pattern = stripe_pattern(&WHITE, &BLACK);
        let c = pattern
            .stripe_at_object(&object, &point(1.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern
            .stripe_at_object(&object, &point(1.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = sphere();
        object.set_transform(&scaling(2.0, 2.0, 2.0));
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern
            .stripe_at_object(&object, &point(2.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }
}
