use std::ops::{Index, IndexMut, Mul};

use crate::{float::*, tuple::*};

pub fn matrix2<R: Into<MatrixRow<2>>>(t1: R, t2: R) -> Matrix2 {
    Matrix2::new(t1.into(), t2.into())
}

pub fn matrix3<R: Into<MatrixRow<3>>>(t1: R, t2: R, t3: R) -> Matrix3 {
    Matrix3::new(t1.into(), t2.into(), t3.into())
}

pub fn matrix4<R: Into<MatrixRow<4>>>(t1: R, t2: R, t3: R, t4: R) -> Matrix {
    Matrix::new(t1.into(), t2.into(), t3.into(), t4.into())
}

pub fn matrix<R: Into<MatrixRow<4>>>(t1: R, t2: R, t3: R, t4: R) -> Matrix {
    Matrix::new(t1.into(), t2.into(), t3.into(), t4.into())
}

const IDENTITY_MATRIX: Matrix = Matrix([
    MatrixRow([1.0, 0.0, 0.0, 0.0]),
    MatrixRow([0.0, 1.0, 0.0, 0.0]),
    MatrixRow([0.0, 0.0, 1.0, 0.0]),
    MatrixRow([0.0, 0.0, 0.0, 1.0]),
]);

pub type MatrixIndex = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct MatrixRow<const LENGTH: usize>([Float; LENGTH]);

impl IntoIterator for MatrixRow<2> {
    type Item = f32;
    type IntoIter = std::array::IntoIter<Float, 2>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for MatrixRow<3> {
    type Item = f32;
    type IntoIter = std::array::IntoIter<Float, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl IntoIterator for MatrixRow<4> {
    type Item = f32;
    type IntoIter = std::array::IntoIter<Float, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<[Float; 2]> for MatrixRow<2> {
    fn from(value: [Float; 2]) -> Self {
        Self(value)
    }
}

impl From<[Float; 3]> for MatrixRow<3> {
    fn from(value: [Float; 3]) -> Self {
        Self(value)
    }
}

impl From<[Float; 4]> for MatrixRow<4> {
    fn from(value: [Float; 4]) -> Self {
        Self(value)
    }
}

impl PartialEq for MatrixRow<2> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .into_iter()
            .enumerate()
            .all(|(index, value)| value.equals(&other.0[index]))
    }
}

impl PartialEq for MatrixRow<3> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .into_iter()
            .enumerate()
            .all(|(index, value)| value.equals(&other.0[index]))
    }
}

impl PartialEq for MatrixRow<4> {
    fn eq(&self, other: &Self) -> bool {
        self.0
            .into_iter()
            .enumerate()
            .all(|(index, value)| value.equals(&other.0[index]))
    }
}

pub trait SubMatrix {
    fn set(&mut self, index: MatrixIndex, value: Float);
}

// Ugly hack to make Matrix2 work as submatrix
impl SubMatrix for Float {
    fn set(&mut self, _index: MatrixIndex, value: Float) {
        *self = value;
    }
}

pub trait Inversion<T, R, M>
where
    T: SubMatrix,
    R: IntoIterator<Item = Float>,
    M: SubMatrix,
{
    fn length(&self) -> usize;

    fn empty() -> M;

    fn empty_submatrix() -> T;

    fn row(&self, row: usize) -> R;

    fn get(&self, row: usize, col: usize) -> Float;

    fn submatrix(&self, row: usize, col: usize) -> T {
        let mut m = Self::empty_submatrix();
        let mut row_index = 0;
        for i in 0..self.length() {
            if i == row {
                continue;
            }
            let mut col_index = 0;
            for j in 0..self.length() {
                if j == col {
                    continue;
                }
                m.set((row_index, col_index), self.get(i, j));
                col_index += 1;
            }
            row_index += 1;
        }
        m
    }

    fn minor(&self, row: usize, col: usize) -> Float;

    fn cofactor(&self, row: usize, col: usize) -> Float {
        let minor = self.minor(row, col);
        if (row + col) % 2 != 0 {
            -minor
        } else {
            minor
        }
    }

    fn determinant(&self) -> Float {
        self.row(0)
            .into_iter()
            .enumerate()
            .map(|(col, val)| val * self.cofactor(0, col))
            .sum()
    }

    fn invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Result<M, String> {
        if !self.invertible() {
            return Err("Matrix is not invertible".to_string());
        }
        let mut m2 = Self::empty();
        for row in 0..self.length() {
            for col in 0..self.length() {
                let c = self.cofactor(row, col);
                m2.set((col, row), c / self.determinant());
            }
        }
        Ok(m2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix2([MatrixRow<2>; 2]);

impl Matrix2 {
    const LENGTH: usize = 2;

    pub fn new<R: Into<MatrixRow<2>>>(t1: R, t2: R) -> Self {
        Matrix2([t1.into(), t2.into()])
    }
}

impl SubMatrix for Matrix2 {
    fn set(&mut self, index: MatrixIndex, value: Float) {
        self.0[index.0].0[index.1] = value;
    }
}

impl Inversion<Float, MatrixRow<2>, Matrix2> for Matrix2 {
    fn length(&self) -> usize {
        Self::LENGTH
    }

    fn empty() -> Self {
        Matrix2::new([0.0; Self::LENGTH], [0.0; Self::LENGTH])
    }

    fn empty_submatrix() -> Float {
        0.0
    }

    fn row(&self, row: usize) -> MatrixRow<2> {
        self.0[row]
    }

    fn get(&self, row: usize, col: usize) -> Float {
        self.0[row].0[col]
    }

    fn submatrix(&self, row: usize, col: usize) -> Float {
        self.0[1 - row].0[1 - col]
    }

    fn minor(&self, _row: usize, _col: usize) -> Float {
        0.0
    }

    fn determinant(&self) -> Float {
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];

        (a * d) - (b * c)
    }
}

impl Index<MatrixIndex> for Matrix2 {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0].0[index.1]
    }
}

impl IndexMut<MatrixIndex> for Matrix2 {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.0[index.0].0[index.1]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix3([MatrixRow<3>; 3]);

impl Matrix3 {
    const LENGTH: usize = 3;

    pub fn new<R: Into<MatrixRow<3>>>(t1: R, t2: R, t3: R) -> Self {
        Matrix3([t1.into(), t2.into(), t3.into()])
    }

    fn get(&self, row: usize, col: usize) -> Float {
        self.0[row].0[col]
    }
}

impl Inversion<Matrix2, MatrixRow<3>, Matrix3> for Matrix3 {
    fn length(&self) -> usize {
        Self::LENGTH
    }

    fn empty() -> Self {
        Matrix3::new(
            [0.0; Self::LENGTH],
            [0.0; Self::LENGTH],
            [0.0; Self::LENGTH],
        )
    }

    fn empty_submatrix() -> Matrix2 {
        Matrix2::empty()
    }

    fn row(&self, row: usize) -> MatrixRow<3> {
        self.0[row]
    }

    fn get(&self, row: usize, col: usize) -> Float {
        self.0[row].0[col]
    }

    fn minor(&self, row: usize, col: usize) -> Float {
        self.submatrix(row, col).determinant()
    }
}

impl SubMatrix for Matrix3 {
    fn set(&mut self, index: MatrixIndex, value: Float) {
        self.0[index.0].0[index.1] = value;
    }
}

impl Index<MatrixIndex> for Matrix3 {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0].0[index.1]
    }
}

impl IndexMut<MatrixIndex> for Matrix3 {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.0[index.0].0[index.1]
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix([MatrixRow<4>; 4]);

impl Matrix {
    const LENGTH: usize = 4;

    pub fn new<R: Into<MatrixRow<4>>>(t1: R, t2: R, t3: R, t4: R) -> Self {
        Matrix([t1.into(), t2.into(), t3.into(), t4.into()])
    }

    pub fn transpose(&self) -> Self {
        Matrix::new(
            [self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]],
            [self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]],
            [self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]],
            [self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]],
        )
    }
}

impl Inversion<Matrix3, MatrixRow<4>, Matrix> for Matrix {
    fn length(&self) -> usize {
        Self::LENGTH
    }

    fn empty() -> Self {
        Matrix::new(
            [0.0; Self::LENGTH],
            [0.0; Self::LENGTH],
            [0.0; Self::LENGTH],
            [0.0; Self::LENGTH],
        )
    }

    fn empty_submatrix() -> Matrix3 {
        Matrix3::empty()
    }

    fn row(&self, row: usize) -> MatrixRow<4> {
        self.0[row]
    }

    fn get(&self, row: usize, col: usize) -> Float {
        self.0[row].0[col]
    }

    fn minor(&self, row: usize, col: usize) -> Float {
        self.submatrix(row, col).determinant()
    }
}

impl SubMatrix for Matrix {
    fn set(&mut self, index: MatrixIndex, value: Float) {
        self.0[index.0].0[index.1] = value;
    }
}

impl Index<MatrixIndex> for Matrix {
    type Output = Float;

    fn index(&self, index: MatrixIndex) -> &Self::Output {
        &self.0[index.0].0[index.1]
    }
}

impl IndexMut<MatrixIndex> for Matrix {
    fn index_mut(&mut self, index: MatrixIndex) -> &mut Self::Output {
        &mut self.0[index.0].0[index.1]
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Self::Output {
        let mut m = Matrix::empty();
        for row in 0..Self::LENGTH {
            for col in 0..Self::LENGTH {
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
        let mut t = [0.0; Self::LENGTH];
        for row in 0..Self::LENGTH {
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
        assert_eq!(A * IDENTITY_MATRIX, A);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(IDENTITY_MATRIX * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let A = matrix(
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        );
        assert_eq!(
            A.transpose(),
            matrix(
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            )
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert_eq!(IDENTITY_MATRIX.transpose(), IDENTITY_MATRIX);
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let A = matrix2([1.0, 5.0], [-3.0, 2.0]);
        assert_eq!(A.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let A = matrix3([1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]);
        assert_eq!(A.submatrix(0, 2), matrix2([-3.0, 2.0], [0.0, 6.0],));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let A = matrix(
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        );
        assert_eq!(
            A.submatrix(2, 1),
            matrix3([-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0],)
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let A = matrix3([3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]);
        let B = A.submatrix(1, 0);
        assert_eq!(B.determinant(), 25.0);
        assert_eq!(A.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let A = matrix3([3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]);
        assert_eq!(A.minor(0, 0), -12.0);
        assert_eq!(A.cofactor(0, 0), -12.0);
        assert_eq!(A.minor(1, 0), 25.0);
        assert_eq!(A.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let A = matrix3([1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]);
        assert_eq!(A.cofactor(0, 0), 56.0);
        assert_eq!(A.cofactor(0, 1), 12.0);
        assert_eq!(A.cofactor(0, 2), -46.0);
        assert_eq!(A.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let A = matrix(
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        );
        assert_eq!(A.cofactor(0, 0), 690.0);
        assert_eq!(A.cofactor(0, 1), 447.0);
        assert_eq!(A.cofactor(0, 2), 210.0);
        assert_eq!(A.determinant(), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let A = matrix(
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        );
        assert_eq!(A.determinant(), -2120.0);
        assert!(A.invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let A = matrix(
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        );
        assert_eq!(A.determinant(), 0.0);
        assert!(!A.invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let A = matrix(
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        );
        let B = A.inverse().unwrap();
        assert_eq!(A.determinant(), 532.0);
        assert_eq!(A.cofactor(2, 3), -160.0);
        assert_eq!(B[(3, 2)], -160.0 / 532.0);
        assert_eq!(A.cofactor(3, 2), 105.0);
        assert_eq!(B[(2, 3)], 105.0 / 532.0);
        assert_eq!(
            B,
            matrix(
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            )
        );
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let A = matrix(
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        );
        assert_eq!(
            A.inverse().unwrap(),
            matrix(
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            )
        );
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let A = matrix(
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        );
        assert_eq!(
            A.inverse().unwrap(),
            matrix(
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            )
        );
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let A = matrix(
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 4.0],
            [-6.0, 5.0, -1.0, 1.0],
        );
        let B = matrix(
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        );
        let C = A * B;
        assert_eq!(C * B.inverse().unwrap(), A);
    }

    #[test]
    fn inverting_the_identity_matrix() {
        assert_eq!(
            IDENTITY_MATRIX.inverse().unwrap(),
            matrix(
                [1.0, -0.0, 0.0, -0.0],
                [-0.0, 1.0, -0.0, 0.0],
                [0.0, -0.0, 1.0, -0.0],
                [-0.0, 0.0, -0.0, 1.0],
            )
        );
    }

    #[test]
    fn multiply_a_matrix_with_its_inverse() {
        let A = matrix(
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        );
        let B = A.inverse().unwrap();
        assert_eq!(A * B, IDENTITY_MATRIX.inverse().unwrap());
    }

    #[test]
    fn inverse_of_transpose_of_a_matrix_equals_transpose_of_inverse() {
        let A = matrix(
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        );
        let B = A.transpose().inverse().unwrap();
        let C = A.inverse().unwrap().transpose();
        assert_eq!(B, C);
    }

    #[test]
    fn multiplying_changed_identity_matrix_with_tuple() {
        let mut A = IDENTITY_MATRIX;
        A[(0, 1)] = 1.0;
        let a = tuple(1.0, 2.0, 3.0, 4.0);
        assert_eq!(A * a, tuple(3.0, 2.0, 3.0, 4.0));
    }
}
