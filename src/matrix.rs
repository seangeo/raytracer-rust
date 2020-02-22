// Implements a matrix and operations on matrices.
//
// Only supports square matrices.

use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug)]
pub struct Matrix4x4 {
    elements: [[f64; 4]; 4]
}

impl Matrix4x4 {
    pub fn from_elements(elements: [[f64; 4]; 4]) -> Matrix4x4 {
        Matrix4x4 {
            elements: elements
        }
    }

    pub fn from_elementsi(elements: [[i64; 4]; 4]) -> Matrix4x4 {
        Matrix4x4::from_elements(
            [
                [elements[0][0] as f64, elements[0][1] as f64, elements[0][2] as f64, elements[0][3] as f64],
                [elements[1][0] as f64, elements[1][1] as f64, elements[1][2] as f64, elements[1][3] as f64],
                [elements[2][0] as f64, elements[2][1] as f64, elements[2][2] as f64, elements[2][3] as f64],
                [elements[3][0] as f64, elements[3][1] as f64, elements[3][2] as f64, elements[3][3] as f64]
            ]
        )
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.elements[i][j]
    }

    pub fn row(&self, i: usize) -> [f64; 4] {
        self.elements[i]
    }

    pub fn col(&self, j: usize) -> [f64; 4] {
        [
            self.get(0, j),
            self.get(1, j),
            self.get(2, j),
            self.get(3, j)
        ]
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(&a, &b)| a * b).sum()
}

impl std::cmp::PartialEq<Matrix4x4> for Matrix4x4 {
    fn eq(&self, o: &Matrix4x4) -> bool {
        const EPSILON: f64 = 0.0001;

        self.elements.iter().flatten().
            zip(o.elements.iter().flatten()).
            all(|(x, y)| (x - y).abs() < EPSILON)
    }
}

impl std::ops::Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, m: Matrix4x4) -> Matrix4x4 {
        let mut result = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = dot(&self.row(i), &m.col(j));
            }
        }

        Matrix4x4::from_elements(result)
    }
}

impl std::ops::Mul<Point> for Matrix4x4 {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        let p = p.as_array();

        Point::new(
            dot(&self.row(0), &p),
            dot(&self.row(1), &p),
            dot(&self.row(2), &p)
        )
    }
}

impl std::ops::Mul<Vector> for Matrix4x4 {
    type Output = Vector;

    fn mul(self, v: Vector) -> Vector {
        let v = v.to_array();

        Vector::new(
            dot(&self.row(0), &v),
            dot(&self.row(1), &v),
            dot(&self.row(2), &v),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point::Point;
    use crate::vector::Vector;

    #[test]
    fn can_create_a_4x4_matrix() {
        let m = Matrix4x4::from_elements(
            [
                [1.0, 2.0, 3.0, 4.0],
                [1.1, 2.1, 3.1, 4.1],
                [1.2, 2.2, 3.2, 4.2],
                [1.3, 2.3, 3.3, 4.3]
            ]);
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
    fn comparing_equal_matrices() {
        let m1 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 4.3]
        ]);
        let m2 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 4.3]
        ]);
        assert_eq!(m1, m2)
    }

    #[test]
    fn comparing_nonequal_matrices() {
        let m1 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 4.3]
        ]);
        let m2 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 5.3]
        ]);
        assert_ne!(m1, m2)
    }

    #[test]
    fn comparing_nearly_equal_matrices() {
        let m1 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 4.3]
        ]);
        let m2 = Matrix4x4::from_elements([
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [1.2, 2.2, 3.2, 4.2],
            [1.3, 2.3, 3.3, 4.30003]
        ]);
        assert_eq!(m1, m2)
    }

    #[test]
    fn can_multiply_two_matrixes() {
        let m1v = [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 8, 7, 6],
            [5, 4, 3, 2]
        ];
        let m2v = [
            [-2, 1, 2, 3],
            [3, 2, 1, -1],
            [4, 3, 6,  5],
            [1, 2, 7, 8]
        ];
        let resultv = [
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42]
        ];
        let m1 = Matrix4x4::from_elementsi(m1v);
        let m2 = Matrix4x4::from_elementsi(m2v);
        let result = Matrix4x4::from_elementsi(resultv);

        assert_eq!(result, m1 * m2);
    }

    #[test]
    fn can_multiply_a_matrix_by_a_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::from_elementsi([
            [1, 2, 3, 4],
            [2, 4, 4, 2],
            [8, 6, 4, 1],
            [0, 0, 0, 1]
        ]);
        let result = Point::new(18.0, 24.0, 33.0);
        assert_eq!(result, m * p);
    }

    #[test]
    fn can_multiply_a_matrix_by_a_vector() {
        let p = Vector::new(1.0, 2.0, 3.0);
        let m = Matrix4x4::from_elementsi([
            [1, 2, 3, 4],
            [2, 4, 4, 2],
            [8, 6, 4, 1],
            [0, 0, 0, 1]
        ]);
        let result = Vector::new(14.0, 22.0, 32.0);
        assert_eq!(result, m * p);
    }
}
