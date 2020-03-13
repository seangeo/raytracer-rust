pub use crate::{Intersection, Material, Matrix4x4, Ray, Point, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShapeType {
    Plane,
    Sphere,
    TestShape
}

impl ShapeType {
    pub fn intersects(&self, local_ray: &Ray) -> Vec<f64> {
        match self {
            Self::Plane => Self::plane_intersection(local_ray),
            Self::Sphere => Self::sphere_intersection(local_ray),
            Self::TestShape => vec![]
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self {
            Self::Plane => Vector::new(0.0, 1.0, 0.0),
            Self::Sphere => point - Point::origin(),
            Self::TestShape => Vector::new(0.0, 0.0, 0.0)
        }
    }

    fn plane_intersection(ray: &Ray) -> Vec<f64> {
        if ray.direction.y.abs() < 0.0001 {
            vec![]
        } else {
            vec![-ray.origin.y / ray.direction.y]
        }
    }

    fn sphere_intersection(object_ray: &Ray) -> Vec<f64> {
        let sphere_to_ray = object_ray.origin - Point::origin();

        let a = object_ray.direction.dot(object_ray.direction);
        let b = 2.0 * object_ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            vec![
                (-b - discriminant.sqrt()) / (2.0 * a),
                (-b + discriminant.sqrt()) / (2.0 * a)
            ]
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Shape {
    shape_type: ShapeType,
    pub material: Material,
    pub transform: Matrix4x4,
    inverse: Matrix4x4
}

impl Shape {
    fn new(shape_type: ShapeType) -> Shape {
        Shape{
            shape_type,
            material: Material::new(),
            transform: Matrix4x4::identity(),
            inverse: Matrix4x4::identity()
        }
    }

    pub fn test_shape() -> Shape {
        Self::new(ShapeType::TestShape)
    }

    pub fn plane() -> Shape {
        Self::new(ShapeType::Plane)
    }

    pub fn sphere() -> Shape {
        Self::new(ShapeType::Sphere)
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
        let intersects = self.shape_type.intersects(&object_ray);
        intersects.
            iter().
            map(|t| Intersection{ray: &ray, object: self, t: *t}).
            collect()
    }

    pub fn normal_at(&self, p: Point) -> Vector {
        let object_point = self.inverse * p;
        let object_normal = self.shape_type.normal_at(object_point);
        let world_normal = self.inverse.transpose() * object_normal;

        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests;
