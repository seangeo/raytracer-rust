// extern crate ray_tracer;

use ray_tracer::Tuple;

#[test]
fn tuple_with_w_1_is_a_point() {
    let t = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(t.x, 4.3);
    assert_eq!(t.y, -4.2);
    assert_eq!(t.z, 3.1);
    assert_eq!(t.w, 1.0);
    assert!(t.is_point());
    assert!(!t.is_vector());
}

#[test]
fn tuple_with_w_0_is_a_vector() {
    let t = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(t.x, 4.3);
    assert_eq!(t.y, -4.2);
    assert_eq!(t.z, 3.1);
    assert_eq!(t.w, 0.0);
    assert!(!t.is_point());
    assert!(t.is_vector());
}

#[test]
fn create_point() {
    let p = Tuple::point(1.0, 2.0, 3.0);
    assert_eq!(p, Tuple::new(1.0, 2.0, 3.0, 1.0));
}

#[test]
fn create_vector() {
    let v = Tuple::vector(4.0, 5.0, 6.0);
    assert_eq!(v, Tuple::new(4.0, 5.0, 6.0, 0.0));
}
