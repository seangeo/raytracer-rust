// Implements a matrix and operations on matrices.
//
// Only supports square matrices.

use crate::point::Point;
use crate::vector::Vector;

#[derive(Debug, PartialEq)]
struct Matrix2x2 {
    elements: [[f64; 2]; 2]
}

impl Matrix2x2 {
    fn determinant(&self) -> f64 {
        (self.elements[0][0] * self.elements[1][1])
            - (self.elements[0][1] * self.elements[1][0])
    }
}

#[derive(Debug, PartialEq)]
struct Matrix3x3 {
    elements: [[f64; 3]; 3]
}

impl Matrix3x3 {
    fn from_elements(elements: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3 {
            elements: elements
        }
    }

    #[cfg(test)]
    pub fn from_elementsi(elements: [[i64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3::from_elements(
            [
                [elements[0][0] as f64, elements[0][1] as f64, elements[0][2] as f64],
                [elements[1][0] as f64, elements[1][1] as f64, elements[1][2] as f64],
                [elements[2][0] as f64, elements[2][1] as f64, elements[2][2] as f64]
            ]
        )
    }

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        if (i + j) % 2 == 1 {
            -self.minor(i, j)
        } else {
            self.minor(i, j)
        }
    }

    fn determinant(&self) -> f64 {
        let mut d = 0.0;
        let row = self.elements[0];

        for j in 0..3 {
            d += self.cofactor(0, j) * row[j];
        }

        d
    }

    fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut sub = [[0.0; 2]; 2];

        for i in 0..2 {
            let r = if i >= row { i + 1 } else { i };
            for j in 0..2 {
                let c = if j >= col { j + 1 } else { j };

                sub[i][j] = self.elements[r][c];
            }
        }

        Matrix2x2{elements: sub}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix4x4 {
    elements: [[f64; 4]; 4]
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4::from_elementsi([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1]
        ])
    }

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

    fn cofactor(&self, i: usize, j: usize) -> f64 {
        if (i + j) % 2 == 1 {
            -self.minor(i, j)
        } else {
            self.minor(i, j)
        }
    }

    pub fn determinant(&self) -> f64 {
        let mut d = 0.0;
        let row = self.elements[0];

        for j in 0..4 {
            d += self.cofactor(0, j) * row[j];
        }

        d
    }

    pub fn inverse(&self) -> Option<Self> {
        let d = self.determinant();

        if d == 0.0 {
            None
        } else {
            let mut inv = [[0.0; 4]; 4];

            for i in 0..4 {
                for j in 0..4 {
                    inv[j][i] = self.cofactor(i, j) / d;
                }
            }

            Some(Matrix4x4{elements: inv})
        }
    }

    fn minor(&self, i: usize, j: usize) -> f64 {
        self.submatrix(i, j).determinant()
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut sub = [[0.0; 3]; 3];

        for i in 0..3 {
            let r = if i >= row { i + 1 } else { i };
            for j in 0..3 {
                let c = if j >= col { j + 1 } else { j };

                sub[i][j] = self.elements[r][c];
            }
        }

        Matrix3x3::from_elements(sub)
    }

    pub fn transpose(&self) -> Matrix4x4 {
        Matrix4x4{
            elements: [
                [self.get(0, 0), self.get(1, 0), self.get(2, 0), self.get(3, 0)],
                [self.get(0, 1), self.get(1, 1), self.get(2, 1), self.get(3, 1)],
                [self.get(0, 2), self.get(1, 2), self.get(2, 2), self.get(3, 2)],
                [self.get(0, 3), self.get(1, 3), self.get(2, 3), self.get(3, 3)]
            ]
        }
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
    fn multiplying_a_matrix_by_the_identity_matrix_returns_itself() {
        let m = Matrix4x4::from_elementsi( [
            [0, 1, 2, 4],
            [1, 2, 4, 8],
            [2, 4, 8, 16],
            [4, 8, 16, 32]
        ]);
        assert_eq!(m, m * Matrix4x4::identity())
    }

    #[test]
    fn multiplying_point_by_identity_returns_the_point() {
        let p = Point::new(1.0, 2.0, 3.0);
        assert_eq!(p, Matrix4x4::identity() * p)
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Matrix4x4::from_elementsi([
            [0,9,3,0],
            [9,8,0,8],
            [1,8,5,3],
            [0,0,5,8]
        ]);
        let transposed = Matrix4x4::from_elementsi([
            [0,9,1,0],
            [9,8,8,0],
            [3,0,5,5],
            [0,8,3,8]
        ]);

        assert_eq!(transposed, m.transpose());
    }

    #[test]
    fn transposing_identity() {
        assert_eq!(Matrix4x4::identity(), Matrix4x4::identity().transpose())
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

    #[test]
    fn determinant_of_2x2() {
        let m = Matrix2x2{elements: [[1.0, 5.0], [-3.0, 2.0]]};
        assert_eq!(17.0, m.determinant())
    }

    #[test]
    fn submatrix_of_4x4_matrix() {
        let m4 = Matrix4x4::from_elementsi([
            [-6, 1, 1, 6],
            [-8, 5, 8, 6],
            [-1, 0, 8, 2],
            [-7, 1,-1, 1]
        ]);
        let m3 = Matrix3x3::from_elementsi([
            [-6, 1, 6],
            [-8, 8, 6],
            [-7, -1, 1]
        ]);

        assert_eq!(m3, m4.submatrix(2, 1))
    }

    #[test]
    fn submatrix_of_3x3_mmatrix() {
        let m3 = Matrix3x3::from_elementsi([
            [1,  5, 0],
            [-3, 2, 7],
            [0,  6,-3]
        ]);
        let m2 = Matrix2x2{elements: [[-3.0, 2.0], [0.0, 6.0]]};
        assert_eq!(m2, m3.submatrix(0, 2))
    }

    #[test]
    fn minor_of_3x3_matrix() {
        let m3 = Matrix3x3::from_elementsi([
            [3, 5, 0],
            [2,-1,-7],
            [6,-1, 5]
        ]);

        assert_eq!(25.0, m3.minor(1, 0));
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let m3 = Matrix3x3::from_elementsi([
            [3, 5, 0],
            [2,-1,-7],
            [6,-1, 5]
        ]);
        assert_eq!(-12.0, m3.minor(0, 0));
        assert_eq!(-12.0, m3.cofactor(0, 0));
        assert_eq!(25.0, m3.minor(1, 0));
        assert_eq!(-25.0, m3.cofactor(1, 0));
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let m3 = Matrix3x3::from_elementsi([
            [ 1, 2, 6],
            [-5, 8,-4],
            [ 2, 6, 4]
        ]);
        assert_eq!(56.0, m3.cofactor(0, 0));
        assert_eq!(12.0, m3.cofactor(0, 1));
        assert_eq!(-46.0, m3.cofactor(0, 2));
        assert_eq!(-196.0, m3.determinant());
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let m = Matrix4x4::from_elementsi([
            [-2,-8, 3, 5],
            [-3, 1, 7, 3],
            [1, 2,-9, 6],
            [-6, 7, 7,-9]
        ]);
        assert_eq!(690.0, m.cofactor(0, 0));
        assert_eq!(447.0, m.cofactor(0, 1));
        assert_eq!(210.0, m.cofactor(0, 2));
        assert_eq!(51.0, m.cofactor(0, 3));
        assert_eq!(-4071.0, m.determinant());
    }

    #[test]
    fn inverse_of_non_invertible_matrix_is_none() {
        let m = Matrix4x4::from_elementsi([
            [-4, 2,-2,-3],
            [ 9, 6, 2, 6],
            [ 0,-5, 1,-5],
            [ 0, 0, 0, 0]
        ]);

        assert_eq!(None, m.inverse());
    }

    #[test]
    fn inverse_of_invertible_matrix() {
        let m = Matrix4x4::from_elementsi([
            [-5, 2, 6,-8],
            [1,-5, 1, 8],
            [7, 7,-6,-7],
            [1,-3, 7, 4]
        ]);
        let result = Matrix4x4::from_elements([
            [ 0.21805 , 0.45113 , 0.24060 , -0.04511 ],
            [-0.80827 , -1.45677 , -0.44361 , 0.52068 ],
            [-0.07895 , -0.22368 , -0.05263 , 0.19737 ],
            [-0.52256 , -0.81391 , -0.30075 , 0.30639 ]
        ]);

        assert_eq!(160.0, m.minor(2, 3));
        assert_eq!(-160.0, m.cofactor(2, 3));
        assert_eq!(-24.0, m.cofactor(3, 0));
        assert_eq!(result, m.inverse().unwrap());
    }

    #[test]
    fn matrix_inversion1() {
        let m = Matrix4x4::from_elementsi([
            [8,-5, 9, 2],
            [7,5,6,1],
            [-6, 0, 9, 6],
            [-3, 0,-9,-4]
        ]);

        let result = Matrix4x4::from_elements([
            [-0.15385 , -0.15385 , -0.28205 , -0.53846 ],
            [-0.07692 , 0.12308 , 0.02564 , 0.03077 ],
            [0.35897 , 0.35897 , 0.43590 , 0.92308 ],
            [-0.69231 , -0.69231 , -0.76923 , -1.92308]
        ]);

        assert_eq!(result, m.inverse().unwrap());
    }

    #[test]
    fn matrix_inversion2() {
        let m = Matrix4x4::from_elementsi([
            [9,3,0,9],
            [-5 , -2 , -6 , -3 ],
            [-4, 9, 6, 4],
            [-7, 6, 6, 2]
        ]);

        let result = Matrix4x4::from_elements([
            [-0.04074 , -0.07778 , 0.14444 , -0.22222 ],
            [-0.07778 , 0.03333 , 0.36667 , -0.33333 ],
            [-0.02901 , -0.14630 , -0.10926 , 0.12963 ],
            [0.17778 , 0.06667 , -0.26667 , 0.33333]
        ]);

        assert_eq!(result, m.inverse().unwrap());
    }

    #[test]
    fn multiply_product_by_inverse() {
        let a = Matrix4x4::from_elementsi([
            [3,-9, 7, 3],
            [3,-8, 2,-9],
            [-4, 4, 4, 1],
            [-6, 5,-1, 1]
        ]);
        let b = Matrix4x4::from_elementsi([
            [8,2,2,2],
            [3,-1, 7, 0],
            [7,0,5,4],
            [6,-2, 0, 5]
        ]);

        let c = a * b;
        assert_eq!(a, c * b.inverse().unwrap());
    }
}
