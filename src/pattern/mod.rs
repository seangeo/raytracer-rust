use crate::{Color, Matrix4x4, Point};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PatternType {
    Solid(Color),
    Stripe(Color, Color)
}

impl PatternType {
    pub fn color_at(&self, p: Point) -> Color {
        match self {
            Self::Solid(c) => *c,
            Self::Stripe(c1, c2) => Self::stripe_color_at(*c1, *c2, p)
        }
    }

    fn stripe_color_at(c1: Color, c2: Color, p: Point) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            c1
        } else {
            c2
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Matrix4x4,
    inverse_transform: Matrix4x4
}

impl Pattern {
    pub fn solid(c: Color) -> Pattern {
        Pattern {
            transform: Matrix4x4::identity(),
            inverse_transform: Matrix4x4::identity(),
            pattern_type: PatternType::Solid(c)
        }
    }

    pub fn stripe(c1: Color, c2: Color) -> Pattern {
        Pattern {
            transform: Matrix4x4::identity(),
            inverse_transform: Matrix4x4::identity(),
            pattern_type: PatternType::Stripe(c1, c2)
        }
    }

    pub fn transform(self, transform: Matrix4x4) -> Pattern {
        Pattern {
            transform,
            inverse_transform: transform.inverse().unwrap(),
            ..self
        }
    }

    pub fn color_at(&self, p: Point) -> Color {
        self.pattern_type.color_at(self.inverse_transform * p)
    }
}

#[cfg(test)]
mod tests;
