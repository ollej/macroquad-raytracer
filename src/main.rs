use macroquad::prelude::*;
use std::ops;

const EPSILON: Float = 0.00001;

type Float = f32;

trait FloatExt {
    fn equals(&self, other: &Float) -> bool;
}

impl FloatExt for Float {
    fn equals(&self, other: &Float) -> bool {
        (self - other).abs() < EPSILON
    }
}

#[derive(Debug, Clone, Copy)]
struct Tuple {
    x: Float,
    y: Float,
    z: Float,
    w: Float,
}

type Vector = Tuple;
type Point = Tuple;
type Color = Tuple;

impl Tuple {
    fn new(x: Float, y: Float, z: Float, w: Float) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn point(x: Float, y: Float, z: Float) -> Point {
        Tuple { x, y, z, w: 1.0 }
    }

    fn vector(x: Float, y: Float, z: Float) -> Vector {
        Tuple { x, y, z, w: 0.0 }
    }

    fn color(red: Float, green: Float, blue: Float) -> Color {
        Tuple {
            x: red,
            y: green,
            z: blue,
            w: 0.0,
        }
    }

    fn red(&self) -> Float {
        self.x
    }

    fn green(&self) -> Float {
        self.y
    }

    fn blue(&self) -> Float {
        self.z
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn magnitude(&self) -> Float {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0) + self.w.powf(2.0)).sqrt()
    }

    fn normalize(&self) -> Tuple {
        Tuple {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.z / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    fn dot(&self, other: &Self) -> Float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn as_color(&self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.x,
            g: self.y,
            b: self.z,
            a: self.w,
        }
    }
}

impl Color {
    fn as_byte_strings(&self) -> [String; 3] {
        let red = (self.x * 255.0).round().clamp(0.0, 255.0) as u8;
        let green = (self.y * 255.0).round().clamp(0.0, 255.0) as u8;
        let blue = (self.z * 255.0).round().clamp(0.0, 255.0) as u8;
        [red.to_string(), green.to_string(), blue.to_string()]
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

impl ops::Add<&Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: &Tuple) -> Self::Output {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Self::Output {
        self.add(&other)
    }
}

impl ops::Sub<&Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: &Tuple) -> Self::Output {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Self::Output {
        self.sub(&other)
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<Float> for Tuple {
    type Output = Tuple;

    fn mul(self, other: Float) -> Self::Output {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<Float> for Tuple {
    type Output = Tuple;

    fn div(self, other: Float) -> Self::Output {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Self::Output {
        Color::color(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Self {
        let pixels = vec![color(0.0, 0.0, 0.0); width * height];
        Self {
            width,
            height,
            pixels,
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.pixels[y * self.width + x] = color.to_owned();
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    fn fill(&mut self, color: &Color) {
        for i in 0..self.width {
            for j in 0..self.height {
                self.write_pixel(i, j, color);
            }
        }
    }

    fn as_ppm(&self) -> String {
        let width = self.width;
        let height = self.height;
        let mut output = String::new();
        output.push_str(&format!(
            "P3\n\
            {width} {height}\n\
            255\n"
        ));

        let mut line = String::new();
        for (index, c) in self
            .pixels
            .iter()
            .flat_map(|p| p.as_byte_strings().to_vec())
            .enumerate()
        {
            if line.len() + c.len() >= 70 {
                output.push_str(&line);
                output.push_str("\n");
                line = c;
            } else if (index + 1) % (self.width * 3) == 0 {
                line.push_str(" ");
                line.push_str(&c);
                output.push_str(&line);
                output.push_str("\n");
                line = String::new()
            } else {
                if index % (self.width * 3) != 0 {
                    line.push_str(" ");
                }
                line.push_str(&c);
            }
        }
        output.push_str(&line);
        output
    }
}

fn tuple(x: Float, y: Float, z: Float, w: Float) -> Tuple {
    Tuple::new(x, y, z, w)
}

fn point(x: Float, y: Float, z: Float) -> Tuple {
    Tuple::point(x, y, z)
}

fn vector(x: Float, y: Float, z: Float) -> Tuple {
    Tuple::vector(x, y, z)
}

fn color(red: Float, green: Float, blue: Float) -> Tuple {
    Tuple::color(red, green, blue)
}

fn canvas(width: usize, height: usize) -> Canvas {
    Canvas::new(width, height)
}

#[derive(Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

impl Projectile {
    fn new(position: Point, velocity: Vector) -> Self {
        Self { position, velocity }
    }

    fn has_landed(&self) -> bool {
        self.position.y <= 0.0
    }
}

#[derive(Debug)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Environment {
    fn new(gravity: Vector, wind: Vector) -> Self {
        Self { gravity, wind }
    }

    fn tick(&self, projectile: Projectile) -> Projectile {
        let position = projectile.position + &projectile.velocity;
        let velocity = projectile.velocity + &self.gravity + &self.wind;
        return Projectile::new(position, velocity);
    }
}

macro_rules! assert_eq_float {
    ($left:expr, $right:expr) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(left_val.equals(right_val)) {
                    panic!(
                        r#"assertion failed: `(left == right)`
  left: `{:?}`,
 right: `{:?}`"#,
                        left_val, right_val
                    )
                }
            }
        }
    }};
}

#[cfg(test)]
mod test_chapter_1_maths {
    use super::*;

    #[test]
    fn a_tupe_with_w_0_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 1.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 1.0);
        assert_eq!(a.is_point(), true);
        assert_eq!(a.is_vector(), false);
    }

    #[test]
    fn a_tupe_with_w_1_is_a_point() {
        let a = tuple(4.3, -4.2, 3.1, 0.0);
        assert_eq!(a.x, 4.3);
        assert_eq!(a.y, -4.2);
        assert_eq!(a.z, 3.1);
        assert_eq!(a.w, 0.0);
        assert_eq!(a.is_point(), false);
        assert_eq!(a.is_vector(), true);
    }

    #[test]
    fn point_creates_tuples_with_w1() {
        let p = point(4.0, -4.0, 3.0);
        assert_eq!(p, tuple(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w0() {
        let v = vector(4.0, -4.0, 3.0);
        assert_eq!(v, tuple(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = tuple(3.0, -2.0, 5.0, 1.0);
        let a2 = tuple(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, tuple(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_a_point() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vector() {
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = vector(0.0, 0.0, 0.0);
        let v = vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, tuple(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, tuple(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3_with_negative_values() {
        let v = vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), vector(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq_float!(norm.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq_float!(a.dot(&b), 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn shooting_a_projectile() {
        // projectile starts one unit above the origin.
        // velocity is normalized to 1 unit/tick.
        let mut p = Projectile::new(point(0.0, 1.0, 0.0), vector(1.0, 1.0, 0.0).normalize());
        // gravity -0.1 unit/tick, and wind is -0.01 unit/tick.
        let e = Environment::new(vector(0.0, -0.1, 0.0), vector(-0.01, 0.0, 0.0));
        let mut ticks = 0;
        while !p.has_landed() {
            println!("#{p:?}");
            p = e.tick(p);
            ticks = ticks + 1;
        }
        println!("Reached ground after #{ticks} ticks.");
        assert_eq!(ticks, 17);
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

#[cfg(test)]
mod test_chapter_2_canvas {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = canvas(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert_eq!(pixel, color(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = canvas(10, 20);
        let red = color(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, &red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = canvas(5, 3);
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().take(3).collect::<Vec<&str>>(),
            ["P3", "5 3", "255"]
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, &c1);
        c.write_pixel(2, 1, &c2);
        c.write_pixel(4, 2, &c3);
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().skip(3).take(3).collect::<Vec<&str>>(),
            [
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"
            ]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = canvas(10, 2);
        c.fill(&color(1.0, 0.8, 0.6));
        let ppm = c.as_ppm();

        assert_eq!(
            ppm.lines().skip(3).take(4).collect::<Vec<&str>>(),
            [
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153"
            ]
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = canvas(5, 3);
        let ppm = c.as_ppm();

        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}

#[macroquad::main("Macroquad Ray Tracer")]
async fn main() {
    let width = 80;
    let height = 40;
    let mut image = Image::gen_image_color(width, height, WHITE);
    image.set_pixel(40, 20, tuple(1.0, 0.5, 0.5, 1.0).as_color());
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);

    loop {
        set_default_camera();
        clear_background(PURPLE);
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
