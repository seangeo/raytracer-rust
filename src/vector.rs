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
        assert_eq!(zero - v, -v)
    }
}

