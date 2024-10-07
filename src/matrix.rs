use crate::float::*;

pub fn matrix(t1: MatrixRow, t2: MatrixRow, t3: MatrixRow, t4: MatrixRow) -> Matrix {
    Matrix::new(t1, t2, t3, t4)
}

pub type MatrixRow = [Float; 4];
pub struct Matrix([MatrixRow; 4]);

impl Matrix {
    pub fn new(t1: MatrixRow, t2: MatrixRow, t3: MatrixRow, t4: MatrixRow) -> Self {
        Matrix([t1, t2, t3, t4])
    }

    pub fn get(&self, row: usize, col: usize) -> Float {
        self.0[row][col]
    }
}

#[cfg(test)]
mod test_chapter_1_maths {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = matrix(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        );
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(0, 3), 4.0);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.0);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }
}
