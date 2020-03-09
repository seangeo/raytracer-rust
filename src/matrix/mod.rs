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

    pub fn rotation_x(&self, radians: f64) -> Matrix4x4 {
        rotation_x(radians) * self
    }

    pub fn rotation_y(&self, radians: f64) -> Matrix4x4 {
        rotation_y(radians) * self
    }

    pub fn rotation_z(&self, radians: f64) -> Matrix4x4 {
        rotation_z(radians) * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Matrix4x4 {
        scale(x, y, z) * self
    }

    pub fn shearing(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
        shearing(xy, xz, yx, yz, zx, zy) * self
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Matrix4x4 {
        translate(x, y, z) * self
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
        self * &m
    }
}

impl std::ops::Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, m: &Matrix4x4) -> Matrix4x4 {
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

pub fn translate(dx: f64, dy: f64, dz: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [1.0, 0.0, 0.0, dx],
        [0.0, 1.0, 0.0, dy],
        [0.0, 0.0, 1.0, dz],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotation_x(radians: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, radians.cos(), -radians.sin(), 0.0],
        [0.0, radians.sin(), radians.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotation_y(radians: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [radians.cos(), 0.0, radians.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-radians.sin(), 0.0, radians.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn rotation_z(radians: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [radians.cos(), -radians.sin(), 0.0, 0.0],
        [radians.sin(), radians.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
    Matrix4x4::from_elements([
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix4x4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = Matrix4x4::from_elements([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]);

    orientation * translate(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod tests;
