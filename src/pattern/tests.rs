use super::*;
use crate::{Color, Matrix4x4, Point};

#[test]
fn solid_pattern_returns_the_same_color() {
    let p = Pattern::solid(Color::black());

    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 1.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 2.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 3.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 1.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 2.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 3.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(-2.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(-1.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(1.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(2.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(3.0, 0.0, 0.0)));
}

#[test]
fn stripe_pattern_is_constant_in_y() {
    let p = Pattern::stripe(Color::black(), Color::white());
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 1.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 2.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 3.0, 0.0)));
}

#[test]
fn stripe_pattern_is_constant_in_z() {
    let p = Pattern::stripe(Color::black(), Color::white());
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 1.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 2.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 3.0)));
}

#[test]
fn stripe_pattern_alternates_in_x() {
    let p = Pattern::stripe(Color::black(), Color::white());
    assert_eq!(Color::black(), p.color_at(Point::new(-2.0, 0.0, 0.0)));
    assert_eq!(Color::white(), p.color_at(Point::new(-1.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(0.0, 0.0, 0.0)));
    assert_eq!(Color::white(), p.color_at(Point::new(1.0, 0.0, 0.0)));
    assert_eq!(Color::black(), p.color_at(Point::new(2.0, 0.0, 0.0)));
    assert_eq!(Color::white(), p.color_at(Point::new(3.0, 0.0, 0.0)));
}

#[test]
fn stripes_with_a_pattern_transform() {
    let t = Matrix4x4::identity().scale(2.0, 2.0, 2.0);
    let p = Pattern::stripe(Color::black(), Color::white()).transform(t);

    assert_eq!(Color::black(), p.color_at(Point::new(1.9, 1.0, 1.0)));
    assert_eq!(Color::white(), p.color_at(Point::new(2.0, 1.0, 1.0)));
}

