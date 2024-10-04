use macroquad::prelude::*;
use std::ops;

const EPSILON: float = 0.00001;

type float = f32;

trait floatExt {
    fn equals(&self, other: &float) -> bool;
}

impl floatExt for float {
    fn equals(&self, other: &float) -> bool {
        (self - other).abs() < EPSILON
    }
}

#[derive(Debug)]
struct Tuple {
    x: float,
    y: float,
    z: float,
    w: float,
}

impl Tuple {
    fn new(x: float, y: float, z: float, w: float) -> Tuple {
        Tuple { x, y, z, w }
    }

    fn point(x: float, y: float, z: float) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    fn vector(x: float, y: float, z: float) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn magnitude(&self) -> float {
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

    fn dot(&self, other: &Self) -> float {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross(&self, other: &Self) -> Self {
        Self::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn as_color(&self) -> Color {
        Color {
            r: self.x,
            g: self.y,
            b: self.z,
            a: self.w,
        }
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

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<float> for Tuple {
    type Output = Tuple;

    fn mul(self, other: float) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl ops::Div<float> for Tuple {
    type Output = Tuple;

    fn div(self, other: float) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

fn tuple(x: float, y: float, z: float, w: float) -> Tuple {
    Tuple::new(x, y, z, w)
}

fn point(x: float, y: float, z: float) -> Tuple {
    Tuple::point(x, y, z)
}

fn vector(x: float, y: float, z: float) -> Tuple {
    Tuple::vector(x, y, z)
}

#[cfg(test)]
mod test_tuple {
    use super::*;

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
