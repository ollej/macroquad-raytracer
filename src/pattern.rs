use crate::{color::*, matrix::*, object::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Texture {
    Striped(Color, Color),
}

impl Texture {
    pub fn color_at(&self, point: &Point) -> Color {
        match self {
            Texture::Striped(a, b) => {
                if point.x.floor() % 2.0 == 0.0 {
                    *a
                } else {
                    *b
                }
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pattern {
    pub transform: Matrix,
    pub texture: Texture,
}

impl Pattern {
    pub fn new(a: Color, b: Color) -> Pattern {
        Pattern {
            transform: IDENTITY_MATRIX,
            texture: Texture::Striped(a, b),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }

    pub fn pattern_at_object(&self, object: &Object, point: &Point) -> Result<Color, String> {
        let object_point = object.transform.inverse()? * point;
        let pattern_point = self.transform.inverse()? * object_point;
        Ok(self.texture.color_at(&pattern_point))
    }
}

pub fn stripe_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(a.to_owned(), b.to_owned())
}

pub fn pattern_at_shape(texture: &Texture, point: &Point) -> Color {
    texture.color_at(point)
}

#[cfg(test)]
mod test_chapter_10_pattern {
    #![allow(non_snake_case)]

    use super::*;

    use crate::sphere::*;

    #[test]
    fn creating_a_stripe_texture() {
        let texture = Texture::Striped(WHITE, BLACK);
        let Texture::Striped(a, b) = texture;
        assert_eq!(a, WHITE);
        assert_eq!(b, BLACK);
    }

    #[test]
    fn a_stripe_texture_is_constant_in_y() {
        let texture = Texture::Striped(WHITE, BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 2.0, 0.0)), WHITE);

        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 2.0, 0.0)), WHITE);
    }

    #[test]
    fn a_stripe_texture_is_constant_in_z() {
        let texture = Texture::Striped(WHITE, BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 0.0, 2.0)), WHITE);
    }

    #[test]
    fn a_stripe_texture_alternates_in_x() {
        let texture = Texture::Striped(WHITE, BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern_at_shape(&texture, &point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern_at_shape(&texture, &point(-1.1, 0.0, 0.0)), WHITE);
    }

    #[test]
    fn pattern_with_an_object_transformation() {
        let mut object = sphere();
        object.set_transform(&scaling(2.0, 2.0, 2.0));
        let pattern = stripe_pattern(&WHITE, &BLACK);
        let c = pattern
            .pattern_at_object(&object, &point(1.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = sphere();
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(scaling(2.0, 2.0, 2.0));
        let c = pattern
            .pattern_at_object(&object, &point(1.5, 0.0, 0.0))
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
            .pattern_at_object(&object, &point(2.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }
}
