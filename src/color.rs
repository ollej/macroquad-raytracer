use std::ops;

use crate::{float::*, tuple::*};

pub fn color(red: Float, green: Float, blue: Float) -> Color {
    Color::color(red, green, blue)
}

pub type Color = Tuple;

pub const BLACK: Color = Color {
    x: 0.,
    y: 0.,
    z: 0.,
    w: 0.,
};

pub const WHITE: Color = Color {
    x: 1.,
    y: 1.,
    z: 1.,
    w: 0.,
};

impl Color {
    pub fn color(red: Float, green: Float, blue: Float) -> Color {
        Tuple {
            x: red,
            y: green,
            z: blue,
            w: 0.0,
        }
    }

    pub fn as_byte_strings(&self) -> [String; 3] {
        let red = (self.x * 255.0).round().clamp(0.0, 255.0) as u8;
        let green = (self.y * 255.0).round().clamp(0.0, 255.0) as u8;
        let blue = (self.z * 255.0).round().clamp(0.0, 255.0) as u8;
        [red.to_string(), green.to_string(), blue.to_string()]
    }

    pub fn red(&self) -> Float {
        self.x
    }

    pub fn green(&self) -> Float {
        self.y
    }

    pub fn blue(&self) -> Float {
        self.z
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::color(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

#[cfg(test)]
mod test_chapter_2_colors {
    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = color(-0.5, 0.4, 1.7);
        assert_eq!(c.red(), -0.5);
        assert_eq!(c.green(), 0.4);
        assert_eq!(c.blue(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
    }
}
