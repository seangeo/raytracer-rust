use super::*;
use crate::{Point, Vector, Color, PointLight, Pattern};

#[test]
fn eye_between_light_and_surface() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = Color::new(1.9, 1.9, 1.9);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, false))
}

#[test]
fn eye_between_light_and_surface_offset_45_deg() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = Color::new(1.0, 1.0, 1.0);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, false))
}

#[test]
fn eye_between_light_and_surface_light_offset_45_deg() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = Color::new(0.7364, 0.7364, 0.7364);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, false))
}

#[test]
fn eye_in_path_of_light() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, -(2_f64.sqrt() / 2.0), -(2_f64.sqrt() / 2.0));
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
    let result = Color::new(1.6364, 1.6364, 1.6364);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, false))
}

#[test]
fn light_behind_the_surface() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
    let result = Color::new(0.1, 0.1, 0.1);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, false))
}

#[test]
fn light_for_surface_in_shadow() {
    let m = Material::new();
    let position = Point::origin();
    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
    let result = Color::new(0.1, 0.1, 0.1);

    assert_eq!(result, m.lighting(light, position, position, eyev, normalv, true));
}

#[test]
fn lighting_with_a_pattern() {
    let p = Pattern::stripe(Color::white(), Color::black());
    let m = Material::new().
        ambient(1.0).
        diffuse(0.0).
        specular(0.0).
        pattern(p);

    let eyev = Vector::new(0.0, 0.0, -1.0);
    let normalv = Vector::new(0.0, 0.0, -1.0);
    let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());

    assert_eq!(Color::white(), m.lighting(light, Point::new(0.9, 0.0, 0.0), Point::new(0.9, 0.0, 0.0), eyev, normalv, false));
    assert_eq!(Color::black(), m.lighting(light, Point::new(1.1, 0.0, 0.0), Point::new(1.0, 0.0, 0.0), eyev, normalv, false));
}
