use crate::{Point, Ray};

pub struct Sphere {

}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { }
    }

    pub fn intersects(&self, ray: Ray) -> Option<(f64,f64)> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            Some((t1, t2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector};

    #[test]
    fn ray_sphere_intersection_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        assert_eq!(Some((4.0, 6.0)), s.intersects(r));
    }

    #[test]
    fn ray_sphere_intersection_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        assert_eq!(Some((5.0, 5.0)), s.intersects(r));
    }

    #[test]
    fn ray_missses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        assert_eq!(None, s.intersects(r));
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        assert_eq!(Some((-1.0, 1.0)), s.intersects(r));
    }

    #[test]
    fn ray_originates_after_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();

        assert_eq!(Some((-6.0, -4.0)), s.intersects(r));
    }
}
