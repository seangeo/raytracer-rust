pub use crate::{Ray, Point};

pub enum ShapeType {
    Sphere
}

pub struct Shape {
    pub shape_type: ShapeType
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape{ shape_type: ShapeType::Sphere }
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
mod tests;
