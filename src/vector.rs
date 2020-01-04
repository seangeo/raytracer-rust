//
#[derive(Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    // Creates a new vector from 3 elements.
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
}

// impl std::ops::Add<&Tuple> for Tuple {
//     type Output = Tuple;

//     fn add(self, other: &Tuple) -> Tuple {
//         Tuple{
//             x: self.x + other.x,
//             y: self.y + other.y,
//             z: self.z + other.z,
//             w: self.w + other.w
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_a_new_vector() {
        let t = Vector::new(4.3, -4.2, 3.1);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -4.2);
        assert_eq!(t.z, 3.1);
    }
}

