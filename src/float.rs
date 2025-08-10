pub const EPSILON: Float = 0.0001;

pub type Float = f64;
pub const PI: Float = std::f64::consts::PI;
pub const INFINITY: Float = std::f64::INFINITY;
pub const NEG_INFINITY: Float = std::f64::NEG_INFINITY;

pub trait FloatExt {
    fn equals(&self, other: &Float) -> bool;
    fn sqrt(value: Float) -> Float;
}

impl FloatExt for Float {
    fn equals(&self, other: &Float) -> bool {
        (self.is_infinite() && other.is_infinite() && self.signum() == other.signum())
            || (self - other).abs() < EPSILON
    }

    fn sqrt(value: Float) -> Float {
        f64::sqrt(value)
    }
}

#[cfg(test)]
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
pub(crate) use assert_eq_float;
