use super::*;
use crate::{Color, Point};

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
