use crate::{Color, Point, PointLight, Shape, Material, Matrix4x4, Intersection, Ray};
use crate::intersection;

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

    pub fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.intersect(&ray);

        match intersection::hit(&intersections) {
            None => Color::black(),
            Some(hit) => hit.lighting(self.light, self.is_shadowed(hit.over_point()))
        }
    }

    pub fn intersect<'a>(&'a self, r: &'a Ray) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();

        for object in &self.objects {
            intersections.extend(object.intersects(r));
        }

        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    pub fn is_shadowed(&self, p: Point) -> bool {
        let point_to_light = self.light.position - p;
        let distance_to_light = point_to_light.magnitude();
        let ray_to_light = Ray::new(p, point_to_light.normalize());

        match intersection::hit(&self.intersect(&ray_to_light)) {
            None => false,
            Some(hit) => {
                hit.t < distance_to_light
            }
        }
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
        let xs: Vec<Intersection> = w.intersect(&r);
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }

    #[test]
    fn color_at_for_miss() {
        let w = World::default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        assert_eq!(Color::black(), w.color_at(r));
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = Color::new(0.38066, 0.47583, 0.2855);
        assert_eq!(c, w.color_at(r));
    }

    #[test]
    fn color_at_inner_sphere() {
        let w = World::default_world();
        let w = World {
            objects: vec![
                w.objects[0].material(w.objects[0].material.ambient(1.0)),
                w.objects[1].material(w.objects[1].material.ambient(1.0)),
            ],
            ..w
        };

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        assert_eq!(Color::white(), w.color_at(r));
    }

    #[test]
    fn color_when_hit_in_shadow() {
        let w = World {
            light: PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white()),
            objects: vec![
                Shape::sphere(),
                Shape::sphere().transform(Matrix4x4::identity().translate(0.0, 0.0, 10.0))
            ]
        };

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        assert_eq!(Color::new(0.1, 0.1, 0.1), w.color_at(r));

    }

    #[test]
    fn no_shadow_when_nothing_between_point_and_light() {
        let w = World::default_world();
        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(false, w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_there_is_object_between_point_and_light() {
        let w = World::default_world();
        let p = Point::new(10.0, -10.0, 10.0);
        assert_eq!(true, w.is_shadowed(p));
    }
}
