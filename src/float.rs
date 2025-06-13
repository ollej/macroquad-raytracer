pub const EPSILON: Float = 0.0001;

pub type Float = f32;

pub trait FloatExt {
    fn equals(&self, other: &Float) -> bool;
}

impl FloatExt for Float {
    fn equals(&self, other: &Float) -> bool {
        (self - other).abs() < EPSILON
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
