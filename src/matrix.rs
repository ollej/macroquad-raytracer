use std::ops::Index;

use crate::float::*;

pub fn matrix2(t1: MatrixRow2, t2: MatrixRow2) -> Matrix2 {
    Matrix2::new(t1, t2)
}

pub fn matrix3(t1: MatrixRow3, t2: MatrixRow3, t3: MatrixRow3) -> Matrix3 {
    Matrix3::new(t1, t2, t3)
}

pub fn matrix4(t1: MatrixRow4, t2: MatrixRow4, t3: MatrixRow4, t4: MatrixRow4) -> Matrix4 {
    Matrix4::new(t1, t2, t3, t4)
}

pub fn matrix(t1: MatrixRow4, t2: MatrixRow4, t3: MatrixRow4, t4: MatrixRow4) -> Matrix4 {
    Matrix4::new(t1, t2, t3, t4)
}

pub type MatrixIndex = (usize, usize);
pub type MatrixRow2 = [Float; 2];
pub type MatrixRow3 = [Float; 3];
pub type MatrixRow4 = [Float; 4];

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2([MatrixRow2; 2]);

impl Matrix2 {
    pub fn new(t1: MatrixRow2, t2: MatrixRow2) -> Self {
        Matrix2([t1, t2])
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3([MatrixRow3; 3]);

impl Matrix3 {
    pub fn new(t1: MatrixRow3, t2: MatrixRow3, t3: MatrixRow3) -> Self {
        Matrix3([t1, t2, t3])
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix4([MatrixRow4; 4]);

impl Matrix4 {
    pub fn new(t1: MatrixRow4, t2: MatrixRow4, t3: MatrixRow4, t4: MatrixRow4) -> Self {
        Matrix4([t1, t2, t3, t4])
    }
}

impl Index<MatrixIndex> for Matrix2 {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl Index<MatrixIndex> for Matrix3 {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl Index<MatrixIndex> for Matrix4 {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

#[cfg(test)]
mod test_chapter_3_matrices {
    #![allow(non_snake_case)]

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        );
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 3)], 4.0);
        assert_eq!(m[(1, 0)], 5.5);
        assert_eq!(m[(1, 2)], 7.5);
        assert_eq!(m[(2, 2)], 11.0);
        assert_eq!(m[(3, 0)], 13.5);
        assert_eq!(m[(3, 2)], 15.5);
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = matrix2([-3.0, 5.0], [1.0, -2.0]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(0, 1)], 5.0);
        assert_eq!(m[(1, 0)], 1.0);
        assert_eq!(m[(1, 1)], -2.0);
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = matrix3([-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [1.0, -2.0, 1.0]);
        assert_eq!(m[(0, 0)], -3.0);
        assert_eq!(m[(1, 1)], -2.0);
        assert_eq!(m[(2, 2)], 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let A = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let B = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        assert_eq!(A, B);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let A = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let B = matrix(
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        );
        assert_ne!(A, B);
    }
}
