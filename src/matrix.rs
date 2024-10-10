use std::ops::{Index, IndexMut, Mul};

use crate::{float::*, tuple::*};

pub fn matrix2(t1: MatrixRow2, t2: MatrixRow2) -> Matrix2 {
    Matrix2::new(t1, t2)
}

pub fn matrix3(t1: MatrixRow3, t2: MatrixRow3, t3: MatrixRow3) -> Matrix3 {
    Matrix3::new(t1, t2, t3)
}

pub fn matrix4(t1: MatrixRow, t2: MatrixRow, t3: MatrixRow, t4: MatrixRow) -> Matrix {
    Matrix::new(t1, t2, t3, t4)
}

pub fn matrix(t1: MatrixRow, t2: MatrixRow, t3: MatrixRow, t4: MatrixRow) -> Matrix {
    Matrix::new(t1, t2, t3, t4)
}

pub fn identity_matrix() -> Matrix {
    Matrix::identity()
}

pub type MatrixIndex = (usize, usize);
pub type MatrixRow2 = [Float; 2];
pub type MatrixRow3 = [Float; 3];
pub type MatrixRow = [Float; 4];

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
pub struct Matrix([MatrixRow; 4]);

impl Matrix {
    pub fn new(t1: MatrixRow, t2: MatrixRow, t3: MatrixRow, t4: MatrixRow) -> Self {
        Matrix([t1, t2, t3, t4])
    }

    pub fn empty() -> Self {
        Matrix::new([0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4])
    }

    pub fn identity() -> Self {
        Matrix::new(
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        )
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

impl Index<MatrixIndex> for Matrix {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        let mut m = Matrix::empty();
        for row in 0..4 {
            for col in 0..4 {
                m[(row, col)] = self[(row, 0)] * other[(0, col)]
                    + self[(row, 1)] * other[(1, col)]
                    + self[(row, 2)] * other[(2, col)]
                    + self[(row, 3)] * other[(3, col)];
            }
        }
        m
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Self::Output {
        let mut t = [0.0; 4];
        for row in 0..4 {
            t[row] = self[(row, 0)] * other.x
                + self[(row, 1)] * other.y
                + self[(row, 2)] * other.z
                + self[(row, 3)] * other.w;
        }
        Tuple::new(t[0], t[1], t[2], t[3])
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

    #[test]
    fn multiplying_two_matrices() {
        let A = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        );
        let B = matrix(
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        );
        assert_eq!(
            A * B,
            matrix(
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            )
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let A = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        );
        let b = tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(A * b, tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let A = matrix(
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        );
        assert_eq!(A * identity_matrix(), A);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(identity_matrix() * a, a);
    }
}
