use super::*;
use crate::{Point, Vector};

#[test]
fn ray_sphere_intersection_at_two_points() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    assert_eq!(Some((4.0, 6.0)), s.intersects(r));
}

#[test]
fn ray_sphere_intersection_at_tangent() {
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    assert_eq!(Some((5.0, 5.0)), s.intersects(r));
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

    assert_eq!(Some((-1.0, 1.0)), s.intersects(r));
}

#[test]
fn ray_originates_after_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    assert_eq!(Some((-6.0, -4.0)), s.intersects(r));
}
