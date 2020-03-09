pub use crate::{Intersection, Material, Matrix4x4, Ray, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShapeType {
    Sphere
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub material: Material,
    pub transform: Matrix4x4,
    inverse: Matrix4x4
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape{
            shape_type: ShapeType::Sphere,
            material: Material::new(),
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity()
        }
    }

    pub fn material(self, material: Material) -> Shape {
        Shape {
            material,
            ..self
        }
    }

    pub fn transform(self, transform: Matrix4x4) -> Shape {
        Shape{
            transform,
            inverse: transform.inverse().unwrap(),
            ..self
        }
    }

    pub fn intersects<'a>(&'a self, ray: &'a Ray) -> Vec<Intersection> {
        let object_ray = ray.transform(self.inverse);
        let sphere_to_ray = object_ray.origin - Point::origin();

        let a = object_ray.direction.dot(object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec![
                Intersection{ray: &ray, object: self, t: t1},
                Intersection{ray: &ray, object: self, t: t2}
            ]
        }
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let object_point = self.inverse * p;
        let object_normal = object_point - Point::origin();
        let world_normal = self.inverse.transpose() * object_normal;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests;
