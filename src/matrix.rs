// Implements a matrix and operations on matrices.
//
// Only supports square matrices.

#[derive(Debug)]
pub struct Matrix {
    pub size: usize,
    elements: Vec<f64>
}

impl Matrix {
    pub fn from_elements(elements: &[f64]) -> Matrix {
        if !vec![4, 9, 16].contains(&(elements.len() as i32)) {
            panic!("Matrix must have 4, 9, or 16 elements");
        }

        Matrix {
            size: (elements.len() as f64).sqrt() as usize,
            elements: Vec::from(elements)
        }
    }

    pub fn from_elementsi(elements: &[i64]) -> Matrix {
        let elements: Vec<f64> = elements.iter().map(|&e| e as f64).collect();

        Matrix::from_elements(&elements)
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.elements[self.size * i + j]
    }

    pub fn row(&self, i: usize) -> Vec<f64> {
        let mut result = vec![];

        for j in 0..self.size {
            result.push(self.get(i, j));
        }

        result
    }

    pub fn col(&self, j: usize) -> Vec<f64> {
        let mut result = vec![];

        for i in 0..self.size {
            result.push(self.get(i, j));
        }

        result
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(&a, &b)| a * b).sum()

}

impl std::cmp::PartialEq<Matrix> for Matrix {
    fn eq(&self, o: &Matrix) -> bool {
        const EPSILON: f64 = 0.0001;

        if self.size != o.size {
            return false
        } else {
            self.elements.
                iter().
                zip(o.elements.iter()).
                all(|(&x, &y)| (x - y).abs() < EPSILON)
        }
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, m: Matrix) -> Matrix {
        let mut result: Vec<f64> = Vec::new();

        for i in 0..self.size {
            for j in 0..self.size {
                result.push(dot(&self.row(i), &m.col(j)));
            }
        }

        Matrix::from_elements(&result)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_matrix() {
        let elements = vec![1.0, 2.0, 3.0, 4.0];
        let m = Matrix::from_elements(&elements);
        assert_eq!(2, m.size);
        assert_eq!(1.0, m.get(0, 0));
        assert_eq!(2.0, m.get(0, 1));
        assert_eq!(3.0, m.get(1, 0));
        assert_eq!(4.0, m.get(1, 1));
    }

    #[test]
    fn can_create_a_4x4_matrix() {
        let m = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0, 1.1, 2.1, 3.1, 4.1, 1.2, 2.2, 3.2, 4.2, 1.3, 2.3, 3.3, 4.3]);
        assert_eq!(4, m.size);
        assert_eq!(1.0, m.get(0, 0));
        assert_eq!(2.0, m.get(0, 1));
        assert_eq!(3.0, m.get(0, 2));
        assert_eq!(4.0, m.get(0, 3));
        assert_eq!(1.1, m.get(1, 0));
        assert_eq!(2.1, m.get(1, 1));
        assert_eq!(3.1, m.get(1, 2));
        assert_eq!(4.1, m.get(1, 3));
        assert_eq!(1.2, m.get(2, 0));
        assert_eq!(2.2, m.get(2, 1));
        assert_eq!(3.2, m.get(2, 2));
        assert_eq!(4.2, m.get(2, 3));
        assert_eq!(1.3, m.get(3, 0));
        assert_eq!(2.3, m.get(3, 1));
        assert_eq!(3.3, m.get(3, 2));
        assert_eq!(4.3, m.get(3, 3));
    }

    #[test]
    #[should_panic]
    fn fails_if_elements_arent_square() {
        Matrix::from_elements(&vec![1.0, 2.0]);
    }

    #[test]
    fn comparing_equal_matrices() {
        let m1 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m1, m2)
    }

    #[test]
    fn comparing_nonequal_matrices() {
        let m1 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 5.0]);
        assert_ne!(m1, m2)
    }

    #[test]
    fn comparing_nearly_equal_matrices() {
        let m1 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0]);
        let m2 = Matrix::from_elements(&vec![1.0, 2.0, 3.0, 4.0001]);
        assert_eq!(m1, m2)
    }

    #[test]
    fn can_multiply_two_matrixes() {
        let m1v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 8, 7, 6, 5, 4, 3, 2];
        let m2v = vec![-2, 1, 2, 3, 3, 2, 1, -1, 4, 3, 6,  5, 1, 2, 7, 8];
        let resultv = vec![20, 22, 50, 48, 44, 54, 114, 108, 40, 58, 110, 102, 16, 26, 46, 42];
        let m1 = Matrix::from_elementsi(&m1v);
        let m2 = Matrix::from_elementsi(&m2v);
        let result = Matrix::from_elementsi(&resultv);

        assert_eq!(result, m1 * m2);
    }

    // #[test]
    // #[should_panic]
    // fn multiplying_different_sized_matrices_fails() {

    // }
}
