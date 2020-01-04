use ray_tracer::Point;
use ray_tracer::Vector;

#[test]
fn can_add_a_vector_to_a_point() {
    let v = Vector::new(1.0, 2.5, 3.5);
    let p = Point::new(1.0, 2.0, 3.0);
    assert_eq!(Point::new(2.0, 4.5, 6.5), p + v);
    assert_eq!(Point::new(2.0, 4.5, 6.5), v + p)
}

#[test]
fn can_add_a_vector_to_a_vector() {
    let v1 = Vector::new(1.0, 2.0, 3.0);
    let v2 = Vector::new(4.0, 5.0, 6.0);
    assert_eq!(Vector::new(5.0, 7.0, 9.0), v1 + v2)
}

#[test]
fn subtracting_two_points() {
    let p1 = Point::new(3.0, 2.0, 1.0);
    let p2 = Point::new(5.0, 6.0, 7.0);
    assert_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2)
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Point::new(3.0, 2.0, 1.0);
    let v = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(Point::new(-2.0, -4.0, -6.0), p - v)
}

#[test]
fn subtracting_two_vectors() {
    let p1 = Vector::new(3.0, 2.0, 1.0);
    let p2 = Vector::new(5.0, 6.0, 7.0);
    assert_eq!(Vector::new(-2.0, -4.0, -6.0), p1 - p2)
}
