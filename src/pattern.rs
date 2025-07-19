use crate::{color::*, tuple::*};

struct Pattern {
    a: Color,
    b: Color,
}

impl Pattern {
    fn stripe_at(&self, point: &Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

fn stripe_pattern(a: Color, b: Color) -> Pattern {
    Pattern { a, b }
}

fn stripe_at(pattern: &Pattern, point: &Point) -> Color {
    pattern.stripe_at(point)
}

#[cfg(test)]
mod test_chapter_7_view_transform {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(pattern.a, WHITE);
        assert_eq!(pattern.b, BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 2.0, 0.0)), WHITE);

        assert_eq!(pattern.stripe_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(&point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = stripe_pattern(WHITE, BLACK);
        assert_eq!(stripe_at(&pattern, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(stripe_at(&pattern, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(stripe_at(&pattern, &point(-1.1, 0.0, 0.0)), WHITE);
    }
}
