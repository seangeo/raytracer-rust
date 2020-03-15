use super::*;
use crate::{Intersection, Material, Matrix4x4, Point, Vector};

#[test]
fn testshape_has_default_material() {
    let s = Shape::test_shape();
    assert_eq!(Material::new(), s.material);
}

#[test]
fn test_shape_can_be_assigned_material() {
    let m = Material::new().ambient(1.0);
    let s = Shape::test_shape().material(m.clone());
    assert_eq!(m, s.material);
}

#[test]
fn test_sphere_default_transform() {
    let s = Shape::test_shape();
    assert_eq!(Matrix4x4::identity(), s.transform);
}

#[test]
fn test_shape_can_have_transform_applied() {
    let m = Matrix4x4::identity().translate(1.0, 2.0, 3.0);
    let s = Shape::test_shape().transform(m);
    assert_eq!(m, s.transform);
}

#[test]
fn ray_sphere_intersection_at_two_points() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = vec![Intersection{ray: &r, object: &s, t: 4.0}, Intersection{ray: &r, object: &s, t: 6.0}];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn ray_sphere_intersection_at_tangent() {
    let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result = vec![Intersection{ray: &r, object: &s, t: 5.0}, Intersection{ray: &r, object: &s, t: 5.0}];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn ray_missses_sphere() {
    let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();
    let result: Vec<Intersection> = vec![];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn ray_originates_inside_a_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = vec![Intersection{ray: &r, object: &s, t: -1.0}, Intersection{ray: &r, object: &s, t: 1.0}];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn ray_originates_after_sphere() {
    let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere();

    let result = vec![Intersection{ray: &r, object: &s, t: -6.0}, Intersection{ray: &r, object: &s, t: -4.0}];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn intersect_scaled_sphere_with_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere().transform(Matrix4x4::identity().scale(2.0, 2.0, 2.0));
    let result = vec![Intersection{ray: &r, object: &s, t: 3.0}, Intersection{ray: &r, object: &s, t: 7.0}];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn intersect_translated_sphere_with_ray() {
    let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
    let s = Shape::sphere().transform(Matrix4x4::identity().translate(5.0, 0.0, 0.0));
    let result: Vec<Intersection> = vec![];

    assert_eq!(result, s.intersects(&r));
}

#[test]
fn normal_at_point_on_x_axis() {
    let s = Shape::sphere();
    let p = Point::new(1.0, 0.0, 0.0);
    let n = Vector::new(1.0, 0.0, 0.0);
    assert_eq!(n, s.normal_at(p));
}

#[test]
fn normal_at_point_on_non_axial_point() {
    let s = Shape::sphere();
    let p = Point::new(3.0_f64.sqrt() / 3.0_f64, 3.0_f64.sqrt() / 3.0_f64, 3.0_f64.sqrt() / 3.0_f64);
    let n = Vector::new(3.0_f64.sqrt() / 3.0_f64, 3.0_f64.sqrt() / 3.0_f64, 3.0_f64.sqrt() / 3.0_f64);
    let r = s.normal_at(p);
    assert_eq!(n, r);
    assert_eq!(r, r.normalize());
}

#[test]
fn normal_at_on_translated_sphere() {
    let s = Shape::sphere().transform(Matrix4x4::identity().translate(0.0, 1.0, 0.0));
    let p = Point::new(0.0, 1.70711, -0.70711);
    let n = Vector::new(0.0, 0.70711, -0.70711);
    assert_eq!(n, s.normal_at(p));
}

#[test]
fn normal_at_on_transformed_sphere() {
    let m = Matrix4x4::identity().rotation_z(std::f64::consts::PI / 5.0).scale(1.0, 0.5, 1.0);
    let s = Shape::sphere().transform(m);
    let p = Point::new(0.0, 2_f64.sqrt() / 2.0, - 2_f64.sqrt() / 2.0);
    let n = Vector::new(0.0, 0.97014, -0.24254);

    assert_eq!(n, s.normal_at(p));
}

//// Planes

#[test]
fn normal_at_on_a_plane() {
    let p = Shape::plane();
    let n = Vector::new(0.0, 1.0, 0.0);
    assert_eq!(n, p.normal_at(Point::new(0.0, 0.0, 0.0)));
    assert_eq!(n, p.normal_at(Point::new(-10.0, 0.0, 1000.0)));
    assert_eq!(n, p.normal_at(Point::new(50.0, 0.0, 50.0)));
}

#[test]
fn plane_intersect_with_parallel_ray() {
    let p = Shape::plane();
    let r = Ray::new(Point::new(0.0, 10.0, 0.0),  Vector::new(0.0, 0.0, 1.0));
    let xs: Vec<Intersection> = vec![];
    assert_eq!(xs, p.intersects(&r));
}

#[test]
fn plane_intersect_with_coplanar_ray() {
    let p = Shape::plane();
    let r = Ray::new(Point::new(0.0, 0.0, 0.0),  Vector::new(0.0, 0.0, 1.0));
    let xs: Vec<Intersection> = vec![];
    assert_eq!(xs, p.intersects(&r));
}

#[test]
fn plane_interesction_from_above() {
    let p = Shape::plane();
    let r = Ray::new(Point::new(0.0, 1.0, 0.0),  Vector::new(0.0, -1.0, 0.0));

    let result = vec![
        Intersection{ray: &r, object: &p, t: 1.0}
    ];

    assert_eq!(result, p.intersects(&r));
}

#[test]
fn plane_interesction_from_below() {
    let p = Shape::plane();
    let r = Ray::new(Point::new(0.0, -1.0, 0.0),  Vector::new(0.0, 1.0, 0.0));

    let result = vec![
        Intersection{ray: &r, object: &p, t: 1.0}
    ];

    assert_eq!(result, p.intersects(&r));
}
