use crate::{color::*, matrix::*, object::*, tuple::*};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Texture {
    Test,
    Striped(Color, Color),
    Gradient(Color, Color),
    Ring(Color, Color),
    Checkers(Color, Color),
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
            Texture::Gradient(a, b) => {
                let distance = b - a;
                let fraction = point.x - point.x.floor();
                *a + distance * fraction
            }
            Texture::Ring(a, b) => {
                if (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() % 2.0 == 0.0 {
                    *a
                } else {
                    *b
                }
            }
            Texture::Checkers(a, b) => {
                if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
                    *a
                } else {
                    *b
                }
            }
            Texture::Test => color(point.x, point.y, point.z),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Pattern {
    pub transform: Matrix,
    pub texture: Texture,
}

impl Pattern {
    pub fn new(transform: Matrix, texture: Texture) -> Pattern {
        Pattern { transform, texture }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }

    pub fn pattern_at_object(&self, object: &Object, point: &Point) -> Result<Color, String> {
        let object_point = object.world_to_object(point)?;
        let pattern_point = self.transform.inverse()? * object_point;
        Ok(self.texture.color_at(&pattern_point))
    }
}

pub fn test_pattern() -> Pattern {
    Pattern::new(IDENTITY_MATRIX, Texture::Test)
}

pub fn stripe_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(
        IDENTITY_MATRIX,
        Texture::Striped(a.to_owned(), b.to_owned()),
    )
}

pub fn gradient_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(
        IDENTITY_MATRIX,
        Texture::Gradient(a.to_owned(), b.to_owned()),
    )
}

pub fn ring_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(IDENTITY_MATRIX, Texture::Ring(a.to_owned(), b.to_owned()))
}

pub fn checkers_pattern(a: &Color, b: &Color) -> Pattern {
    Pattern::new(
        IDENTITY_MATRIX,
        Texture::Checkers(a.to_owned(), b.to_owned()),
    )
}

pub fn test_texture() -> Texture {
    Texture::Test
}

pub fn stripe_texture(a: &Color, b: &Color) -> Texture {
    Texture::Striped(a.to_owned(), b.to_owned())
}

pub fn gradient_texture(a: &Color, b: &Color) -> Texture {
    Texture::Gradient(a.to_owned(), b.to_owned())
}

pub fn ring_texture(a: &Color, b: &Color) -> Texture {
    Texture::Ring(a.to_owned(), b.to_owned())
}

pub fn checkers_texture(a: &Color, b: &Color) -> Texture {
    Texture::Checkers(a.to_owned(), b.to_owned())
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
        if let Texture::Striped(a, b) = texture {
            assert_eq!(a, WHITE);
            assert_eq!(b, BLACK);
        }
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
        object.set_transform(scaling(2.0, 2.0, 2.0));
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
        object.set_transform(scaling(2.0, 2.0, 2.0));
        let mut pattern = stripe_pattern(&WHITE, &BLACK);
        pattern.set_transform(translation(0.5, 0.0, 0.0));
        let c = pattern
            .pattern_at_object(&object, &point(2.5, 0.0, 0.0))
            .unwrap();
        assert_eq!(c, WHITE);
    }

    #[test]
    fn the_default_pattern_transformation() {
        let p = test_pattern();
        assert_eq!(p.transform, IDENTITY_MATRIX);
    }

    #[test]
    fn assigning_a_transformation() {
        let mut p = test_pattern();
        let t = translation(2., 3., 4.);
        p.set_transform(t);
        assert_eq!(p.transform, t);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut shape = sphere();
        shape.set_transform(scaling(2.0, 2.0, 2.0));
        let pattern = test_pattern();
        let c = pattern
            .pattern_at_object(&shape, &point(2.0, 3.0, 4.0))
            .unwrap();
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_transformation() {
        let shape = sphere();
        let mut p = test_pattern();
        p.set_transform(scaling(2.0, 2.0, 2.0));
        let c = p.pattern_at_object(&shape, &point(2.0, 3.0, 4.0)).unwrap();
        assert_eq!(c, color(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let mut shape = sphere();
        shape.set_transform(scaling(2.0, 2.0, 2.0));
        let mut p = test_pattern();
        p.set_transform(translation(0.5, 1.0, 1.5));
        let c = p.pattern_at_object(&shape, &point(2.5, 3.0, 3.5)).unwrap();
        assert_eq!(c, color(0.75, 0.5, 0.25));
    }

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let texture = gradient_texture(&WHITE, &BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            texture.color_at(&point(0.25, 0.0, 0.0)),
            color(0.75, 0.75, 0.75)
        );
        assert_eq!(
            texture.color_at(&point(0.5, 0.0, 0.0)),
            color(0.5, 0.5, 0.5)
        );
        assert_eq!(
            texture.color_at(&point(0.75, 0.0, 0.0)),
            color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn a_ring_should_extend_in_both_x_and_z() {
        let texture = ring_texture(&WHITE, &BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 1.0)), BLACK);
        // 0.708 = just slightly more than √ 2/2;
        assert_eq!(texture.color_at(&point(0.708, 0.0, 0.708)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let texture = checkers_texture(&WHITE, &BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(1.01, 0.0, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let texture = checkers_texture(&WHITE, &BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 1.01, 0.0)), BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let texture = checkers_texture(&WHITE, &BLACK);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(texture.color_at(&point(0.0, 0.0, 1.01)), BLACK);
    }
}

#[cfg(test)]
mod test_chapter_14_group {
    use super::*;

    use crate::{group::*, material::*, sphere::*};

    #[test]
    fn a_pattern_with_parent_transformation() {
        let g1 = &mut empty_group();
        g1.set_transform(translation(3.0, 2.0, 1.0));

        let s = &mut sphere();
        let mut m = Material::default();
        let p = test_pattern();
        m.set_pattern(p);
        s.set_material(&m);
        g1.add_child(s);
        let c = p.pattern_at_object(s, &point(1.0, 2.0, 3.0)).unwrap();

        assert_eq!(c, color(-2.0, 0.0, 2.0));
    }
}
