// Provides a 4 element tuple structure for representing
// points and vectors.
//
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuple {
    // Creates a new tuple from 4 elements.
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    // Checks whether a tuple is a point or not.
    //
    // A tuple is a point if the w component is 1.
    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    // Checks whether a tuple is a vector or not.
    //
    // A tuple is a vector if the w component is 0.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}
