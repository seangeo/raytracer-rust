use crate::{Color, Point};

#[derive(Debug, PartialEq,Copy, Clone)]
pub enum Pattern {
    Solid(Color),
    Stripe(Color, Color)
}

impl Pattern {
    pub fn solid(c: Color) -> Pattern {
        Self::Solid(c)
    }

    pub fn stripe(c1: Color, c2: Color) -> Pattern {
        Self::Stripe(c1, c2)
    }

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

#[cfg(test)]
mod tests;
