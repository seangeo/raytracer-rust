use super::geom::Shape;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape
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
    use super::*;

    #[test]
    fn hit_when_all_intersections_are_positive() {
        let s = Shape::sphere();
        let i1 = Intersection{t: 1.0, object: &s};
        let i2 = Intersection{t: 2.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is).unwrap();

        assert_eq!(is[0], hit)
    }

    #[test]
    fn hit_when_some_intersections_are_negative() {
        let s = Shape::sphere();
        let i1 = Intersection{t: -1.0, object: &s};
        let i2 = Intersection{t: 1.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is).unwrap();

        assert_eq!(is[1], hit)
    }

    #[test]
    fn hit_when_all_intersections_are_negative() {
        let s = Shape::sphere();
        let i1 = Intersection{t: -1.0, object: &s};
        let i2 = Intersection{t: -2.0, object: &s};
        let is = vec![i1, i2];
        let hit = hit(&is);

        assert_eq!(None, hit)
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Shape::sphere();
        let i1 = Intersection{t: 5.0, object: &s};
        let i2 = Intersection{t: 7.0, object: &s};
        let i3 = Intersection{t: -3.0, object: &s};
        let i4 = Intersection{t: 2.0, object: &s};
        let is = vec![i1, i2, i3, i4];
        let hit = hit(&is).unwrap();

        assert_eq!(i4, hit)
    }
}