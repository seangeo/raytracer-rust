use super::*;

#[test]
fn can_create_a_new_vector() {
    let t = Vector::new(4.3, -4.2, 3.1);
    assert_eq!(t.x, 4.3);
    assert_eq!(t.y, -4.2);
    assert_eq!(t.z, 3.1);
}

#[test]
fn can_negate_a_vector() {
    let v = Vector::new(1.0, -2.0, 3.0);
    let zero = Vector::new(0.0, 0.0, 0.0);
    assert_eq!(zero - v, -v);
}

#[test]
fn can_multiply_a_vector_by_a_scalar() {
    let v = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(Vector::new(3.5, -7.0, 10.5), v * 3.5);
}

#[test]
fn can_multiply_a_vector_by_a_scalar_integer() {
    let v = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(Vector::new(-2.0, 4.0, -6.0), v * -2);
}

#[test]
fn can_divide_by_scalar() {
    let v = Vector::new(1.0, -2.0, 3.0);
    assert_eq!(Vector::new(0.5, -1.0, 1.5), v / 2.0)
}

#[test]
fn compute_magnitude_of_unit_on_x() {
    assert_eq!(1.0, Vector::n(1, 0, 0).magnitude())
}

#[test]
fn compute_magnitude_of_unit_on_y() {
    assert_eq!(1.0, Vector::n(0, 1, 0).magnitude())
}

#[test]
fn compute_magnitude_of_unit_on_z() {
    assert_eq!(1.0, Vector::n(0, 0, 1).magnitude())
}

#[test]
fn compute_magnitude_of_positive_vector() {
    assert_eq!(14.0_f64.sqrt(), Vector::n(1, 2, 3).magnitude())
}

#[test]
fn compute_magnitude_of_negative_vector() {
    assert_eq!(14.0_f64.sqrt(), Vector::n(-1, -2, -3).magnitude())
}

#[test]
fn normalize_vector_on_one_axis() {
    assert_eq!(Vector::n(1, 0, 0), Vector::n(4, 0, 0).normalize())
}

#[test]
fn normalize_vector() {
    let expected = Vector::new(1_f64 / 14_f64.sqrt(), 2_f64 / 14_f64.sqrt(), 3_f64 / 14_f64.sqrt());
    assert_eq!(expected, Vector::n(1, 2, 3).normalize())
}

#[test]
fn dot_product() {
    let a = Vector::n(1, 2, 3);
    let b = Vector::n(2, 3, 4);
    assert_eq!(20.0, a.dot(b));
}

#[test]
fn cross_product() {
    let a = Vector::n(1, 2, 3);
    let b = Vector::n(2, 3, 4);
    assert_eq!(Vector::n(-1, 2, -1), a.cross(b));
    assert_eq!(Vector::n(1, -2, 1), b.cross(a))
}

#[test]
fn reflect_vector_at_45_deg() {
    let v = Vector::n(1, -1, 0);
    let n = Vector::n(0, 1, 0);
    let r = Vector::n(1, 1, 0);
    assert_eq!(r, v.reflect(n));
}

#[test]
fn relfect_vector_off_slanted_surface() {
    let v = Vector::n(0, -1, 0);
    let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
    let r = Vector::n(1, 0, 0);
    assert_eq!(r, v.reflect(n))
}
