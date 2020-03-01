use super::*;
use crate::{Intersection, Point, Vector};

#[test]
fn ray_sphere_intersection_at_two_points() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = Some((Intersection{object: &s, t: 4.0}, Intersection{object: &s, t: 6.0}));

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_sphere_intersection_at_tangent() {
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = Some((Intersection{object: &s, t: 5.0}, Intersection{object: &s, t: 5.0}));

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_missses_sphere() {
    let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    assert_eq!(None, s.intersects(r));
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = Some((Intersection{object: &s, t: -1.0}, Intersection{object: &s, t: 1.0}));

    assert_eq!(result, s.intersects(r));
}

#[test]
fn ray_originates_after_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = Some((Intersection{object: &s, t: -6.0}, Intersection{object: &s, t: -4.0}));

    assert_eq!(result, s.intersects(r));
}
