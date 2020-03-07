use crate::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    // Creates a new vector from 3 elements.
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn n(x: i64, y: i64, z: i64) -> Vector {
        Vector::new(x as f64, y as f64, z as f64)
    }

    pub fn cross(&self, v: Vector) -> Vector {
        Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }

    pub fn dot(&self, v: Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector {
        let m = self.magnitude();

        Vector {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m
        }
    }

    pub fn reflect(&self, v: Vector) -> Vector {
        *self - v * 2 * self.dot(v)
    }

    pub fn to_array(self) -> [f64; 4] {
        [self.x, self.y, self.z, 0.0]
    }
}

impl std::ops::Add<Point> for Vector {
    type Output = Point;

    fn add(self, p: Point) -> Point {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, s: f64) -> Vector {
        Vector {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, s: f64) -> Vector {
        Vector {
            x: self.x / s,
            y: self.y / s,
            z: self.z / s
        }
    }
}

impl std::ops::Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, s: i64) -> Vector {
        self * s as f64
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        const EPSILON: f64 = 0.0001;
        (self.x - other.x).abs() < EPSILON &&
            (self.y - other.y).abs() < EPSILON &&
            (self.z - other.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests;
