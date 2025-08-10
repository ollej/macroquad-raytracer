use std::ops;

use crate::float::*;

pub fn color(red: Float, green: Float, blue: Float) -> Color {
    Color::new(red, green, blue)
}

pub const BLACK: Color = Color::new(0., 0., 0.);
pub const WHITE: Color = Color::new(1., 1., 1.);
pub const BANANA_MANIA: Color = Color::new(0.988, 0.89, 0.675);
pub const BRIGHT_SUN: Color = Color::new(0.992, 0.82, 0.263);
pub const BLAZE_ORANGE: Color = Color::new(0.973, 0.396, 0.024);
pub const BURNT_UMBER: Color = Color::new(0.541, 0.145, 0.216);
pub const TEAL_BLUE: Color = Color::new(0.02, 0.22, 0.373);
pub const DEEP_CERULEAN: Color = Color::new(0.012, 0.494, 0.627);
pub const PICTON_BLUE: Color = Color::new(0.298, 0.796, 0.933);

#[derive(Debug, Clone, Copy)]
pub struct Color {
    red: Float,
    green: Float,
    blue: Float,
}

impl Color {
    pub const fn new(red: Float, green: Float, blue: Float) -> Color {
        Color { red, green, blue }
    }

    pub fn as_byte_strings(&self) -> [String; 3] {
        let red = (self.red * 255.0).round().clamp(0.0, 255.0) as u8;
        let green = (self.green * 255.0).round().clamp(0.0, 255.0) as u8;
        let blue = (self.blue * 255.0).round().clamp(0.0, 255.0) as u8;
        [red.to_string(), green.to_string(), blue.to_string()]
    }

    pub fn red(&self) -> Float {
        self.red
    }

    pub fn green(&self) -> Float {
        self.green
    }

    pub fn blue(&self) -> Float {
        self.blue
    }

    pub fn lighten_only_blend(&self, other: Color) -> Color {
        Color::new(
            self.red.max(other.red),
            self.green.max(other.green),
            self.blue.max(other.blue),
        )
    }

    pub fn as_color(&self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.red as f32,
            g: self.green as f32,
            b: self.blue as f32,
            a: 1.0,
        }
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red.equals(&other.red)
            && self.green.equals(&other.green)
            && self.blue.equals(&other.blue)
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

impl ops::Mul<Float> for Color {
    type Output = Color;

    fn mul(self, other: Float) -> Self::Output {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl ops::Add<&Color> for Color {
    type Output = Color;

    fn add(self, other: &Color) -> Self::Output {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        self.add(&other)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.red = self.red + rhs.red;
        self.green = self.green + rhs.green;
        self.blue = self.blue + rhs.blue;
    }
}

impl ops::Sub<&Color> for Color {
    type Output = Color;

    fn sub(self, other: &Color) -> Self::Output {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        self.sub(&other)
    }
}

impl ops::Sub<&Color> for &Color {
    type Output = Color;

    fn sub(self, other: &Color) -> Self::Output {
        (*self).sub(other)
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
