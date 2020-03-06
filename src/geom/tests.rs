use super::*;
use crate::{Intersection, Matrix4x4, Point, Vector};

#[test]
fn sphere_default_transform() {
    let s = Shape::sphere();
    assert_eq!(Matrix4x4::identity(), s.transform);
}

#[test]
fn sphere_can_have_transform_applied() {
    let m = Matrix4x4::identity().translate(1.0, 2.0, 3.0);
    let s = Shape::sphere().transform(m);
    assert_eq!(m, s.transform);
}

#[test]
fn ray_sphere_intersection_at_two_points() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = vec![Intersection{object: &s, t: 4.0}, Intersection{object: &s, t: 6.0}];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_sphere_intersection_at_tangent() {
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = vec![Intersection{object: &s, t: 5.0}, Intersection{object: &s, t: 5.0}];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_missses_sphere() {
    let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result: Vec<Intersection> = vec![];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = vec![Intersection{object: &s, t: -1.0}, Intersection{object: &s, t: 1.0}];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_originates_after_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = vec![Intersection{object: &s, t: -6.0}, Intersection{object: &s, t: -4.0}];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn intersect_scaled_sphere_with_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere().transform(Matrix4x4::identity().scale(2.0, 2.0, 2.0));
    let result = vec![Intersection{object: &s, t: 3.0}, Intersection{object: &s, t: 7.0}];

    assert_eq!(result, s.intersects(r));
}

#[test]
fn intersect_translated_sphere_with_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere().transform(Matrix4x4::identity().translate(5.0, 0.0, 0.0));
    let result: Vec<Intersection> = vec![];

    assert_eq!(result, s.intersects(r));
}
