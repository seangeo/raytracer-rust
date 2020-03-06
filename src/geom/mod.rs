pub use crate::{Intersection, Matrix4x4, Ray, Point};

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub transform: Matrix4x4
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape{
            shape_type: ShapeType::Sphere,
            transform: Matrix4x4::identity()
        }
    }

    pub fn transform(self, t: Matrix4x4) -> Shape {
        Shape{
            transform: t,
            ..self
        }
    }

    pub fn intersects(&self, ray: Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Point::origin();

        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec![
                Intersection{object: self, t: t1},
                Intersection{object: self, t: t2}
            ]
        }
    }
}

#[cfg(test)]
mod tests;
