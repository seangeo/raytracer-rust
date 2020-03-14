use crate::{Color, Point};

trait PatternType {
    fn color_at(&self, p: Point) -> Color;
}

struct Stripe {
    c1: Color,
    c2: Color
}

impl PatternType for Stripe {
    fn color_at(&self, p: Point) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            self.c1
        } else {
            self.c2
        }
    }
}

pub struct Pattern {
    pattern_type: Box<dyn PatternType>
}


impl Pattern {
    pub fn stripe(c1: Color, c2: Color) -> Pattern {
        Pattern {
            pattern_type: Box::new(Stripe { c1, c2 })
        }
    }

    pub fn color_at(&self, p: Point) -> Color {
        self.pattern_type.color_at(p)
    }
}

#[cfg(test)]
mod tests;
