use crate::{Color, Point, PointLight, Shape, Material, Matrix4x4, Intersection, Ray};

#[derive(Debug)]
pub struct World {
    pub light: PointLight,
    pub objects: Vec<Shape>
}

impl World {
    pub fn default_world() -> World {
        World {
            light: PointLight::new(
                       Point::new(-10.0, 10.0, -10.0),
                       Color::new(1.0, 1.0, 1.0)
                   ),
            objects: vec![
                Shape::sphere().
                    material(
                        Material::new().
                            color(Color::new(0.8, 1.0, 0.6)).
                            diffuse(0.7).
                            specular(0.2)),
                Shape::sphere().
                    transform(Matrix4x4::identity().scale(0.5, 0.5, 0.5))
            ]
        }
    }

    pub fn intersect(&self, r: Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();

        for object in &self.objects {
            intersections.extend(object.intersects(r));
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ray, Point, Vector, Intersection};

    #[test]
    fn intersect_world() {
        let w = World::default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs: Vec<Intersection> = w.intersect(r);
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }
}
