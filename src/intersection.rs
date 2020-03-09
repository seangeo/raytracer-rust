use super::geom::{Point, Ray, Shape, Vector};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub ray: &'a Ray,
    pub t: f64,
    pub object: &'a Shape
}

impl Intersection<'_> {
    pub fn point(&self) -> Point {
        self.ray.position(self.t)
    }

    pub fn eyev(&self) -> Vector {
        -self.ray.direction
    }

    pub fn normal(&self) -> Vector {
        self.object.normal_at(self.point())
    }
}

pub fn hit<'a>(intersections: &'a [Intersection]) -> Option<Intersection<'a>> {
    intersections.
        iter().
        fold(None, |hit, &i| {
            if i.t <= 0.0 {
                hit
            } else {
                match hit {
                    None => Some(i),
                    Some(hit) =>  {
                        if hit.t < i.t {
                            Some(hit)
                        } else {
                            Some(i)
                        }
                    }
                }
            }
        })
}

#[cfg(test)]
mod tests {
    use crate::{Ray, Point, Vector};
    use super::*;

    #[test]
    fn hit_when_all_intersections_are_positive() {
        let ray = Ray::new(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let s = Shape::sphere();
        let i1 = Intersection{ray: &ray, t: 1.0, object: &s};
        let i2 = Intersection{ray: &ray, t: 2.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is).unwrap();

        assert_eq!(is[0], hit)
    }

    #[test]
    fn hit_when_some_intersections_are_negative() {
        let ray = Ray::new(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let s = Shape::sphere();
        let i1 = Intersection{ray: &ray, t: -1.0, object: &s};
        let i2 = Intersection{ray: &ray, t: 1.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is).unwrap();

        assert_eq!(is[1], hit)
    }

    #[test]
    fn hit_when_all_intersections_are_negative() {
        let ray = Ray::new(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let s = Shape::sphere();
        let i1 = Intersection{ray: &ray, t: -1.0, object: &s};
        let i2 = Intersection{ray: &ray, t: -2.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is);

        assert_eq!(None, hit)
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let ray = Ray::new(Point::origin(), Vector::new(1.0, 0.0, 0.0));
        let s = Shape::sphere();
        let i1 = Intersection{ray: &ray, t: 5.0, object: &s};
        let i2 = Intersection{ray: &ray, t: 7.0, object: &s};
        let i3 = Intersection{ray: &ray, t: -3.0, object: &s};
        let i4 = Intersection{ray: &ray, t: 2.0, object: &s};
        let is = vec![i1, i2, i3, i4];
        let hit = hit(&is).unwrap();

        assert_eq!(i4, hit)
    }

    #[test]
    fn computing_state_of_an_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection{ray: &ray, t: 4.0, object: &shape};
        assert_eq!(Point::new(0.0, 0.0, -1.0), i.point());
        assert_eq!(Vector::new(0.0, 0.0, -1.0), i.eyev());
        assert_eq!(Vector::new(0.0, 0.0, -1.0), i.normal());
    }
}
