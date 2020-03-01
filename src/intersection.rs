use super::geom::Shape;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape
}
