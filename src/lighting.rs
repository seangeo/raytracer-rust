use crate::{Color, Point};

pub struct PointLight {
    pub position: Point,
    pub intensity: Color
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight{ position, intensity }
    }
}
