use crate::vector::Vector;
use crate::point::Point;
use crate::matrix::Matrix4x4;

#[derive(Copy, Clone)]
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

    pub fn transform(&self, m: Matrix4x4) -> Ray {
        Ray{
            origin: m * self.origin,
            direction: m * self.direction
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix::Matrix4x4;

    #[test]
    fn position_of_ray_at_t() {
        let r =  Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));

        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix4x4::identity().translate(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(Vector::new(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scalinging_a_ray() {
        let r = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Matrix4x4::identity().scale(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(Vector::new(0.0, 3.0, 0.0), r2.direction);
    }
}
