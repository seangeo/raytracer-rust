use crate::vector::Vector;

#[derive(Debug, PartialEq, Copy, Clone)]
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
