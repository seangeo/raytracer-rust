use crate::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point {
    // Creates a new vector from 3 elements.
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn as_array(self) -> [f64; 4] {
        [self.x, self.y, self.z, 1.0]
    }
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, v: Vector) -> Point {
        Point{
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, p: Point) -> Vector {
        Vector {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z
        }
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, p: Vector) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
            z: self.z - p.z
        }
    }
}

impl std::cmp::PartialEq<Point> for Point {
    fn eq(&self, o: &Point) -> bool {
        const EPSILON: f64 = 0.0001;

        (self.x - o.x).abs() < EPSILON &&
            (self.y - o.y).abs() < EPSILON &&
            (self.z - o.z).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_a_new_point() {
        let t = Point::new(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
    }
}
