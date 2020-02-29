use crate::vector::Vector;
use crate::point::Point;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray {origin, direction}
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_of_ray_at_t() {
        let r =  Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }
}
