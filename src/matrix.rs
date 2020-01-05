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

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.elements[self.size * i + j]
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
}
