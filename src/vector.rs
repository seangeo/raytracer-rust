use crate::point::Point;

#[derive(Debug, PartialEq, Copy, Clone)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_a_new_vector() {
        let t = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
    }

    #[test]
    fn can_negate_a_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);
        let zero = Vector::new(0.0, 0.0, 0.0);
        assert_eq!(zero - v, -v);
    }

    #[test]
    fn can_multiply_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(3.5, -7.0, 10.5), v * 3.5);
    }

    #[test]
    fn can_multiply_a_vector_by_a_scalar_integer() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(-2.0, 4.0, -6.0), v * -2);
    }

    #[test]
    fn can_divide_by_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);
        assert_eq!(Vector::new(0.5, -1.0, 1.5), v / 2.0)
    }

    #[test]
    fn compute_magnitude_of_unit_on_x() {
        assert_eq!(1.0, Vector::n(1, 0, 0).magnitude())
    }

    #[test]
    fn compute_magnitude_of_unit_on_y() {
        assert_eq!(1.0, Vector::n(0, 1, 0).magnitude())
    }

    #[test]
    fn compute_magnitude_of_unit_on_z() {
        assert_eq!(1.0, Vector::n(0, 0, 1).magnitude())
    }

    #[test]
    fn compute_magnitude_of_positive_vector() {
        assert_eq!(14.0_f64.sqrt(), Vector::n(1, 2, 3).magnitude())
    }

    #[test]
    fn compute_magnitude_of_negative_vector() {
        assert_eq!(14.0_f64.sqrt(), Vector::n(-1, -2, -3).magnitude())
    }

    #[test]
    fn normalize_vector_on_one_axis() {
        assert_eq!(Vector::n(1, 0, 0), Vector::n(4, 0, 0).normalize())
    }

    #[test]
    fn normalize_vector() {
        let expected = Vector::new(1_f64 / 14_f64.sqrt(), 2_f64 / 14_f64.sqrt(), 3_f64 / 14_f64.sqrt());
        assert_eq!(expected, Vector::n(1, 2, 3).normalize())
    }

    #[test]
    fn dot_product() {
        let a = Vector::n(1, 2, 3);
        let b = Vector::n(2, 3, 4);
        assert_eq!(20.0, a.dot(b));
    }

    #[test]
    fn cross_product() {
        let a = Vector::n(1, 2, 3);
        let b = Vector::n(2, 3, 4);
        assert_eq!(Vector::n(-1, 2, -1), a.cross(b));
        assert_eq!(Vector::n(1, -2, 1), b.cross(a))
    }
}

