pub const EPSILON: Float = 0.00001;

pub type Float = f32;

pub trait FloatExt {
    fn equals(&self, other: &Float) -> bool;
}

impl FloatExt for Float {
    fn equals(&self, other: &Float) -> bool {
        (self - other).abs() < EPSILON
    }
}
