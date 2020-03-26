use crate::{Color, Matrix4x4, Point};

#[derive(Debug, PartialEq, Clone)]
pub enum PatternType {
    Checkers(Box<Pattern>, Box<Pattern>),
    Solid(Color),
    Stripe(Box<Pattern>, Box<Pattern>),
    LinearGradient(Color, Color),
    Ring(Box<Pattern>, Box<Pattern>)
}

impl PatternType {
    pub fn color_at(&self, p: Point) -> Color {
        match self {
            Self::Checkers(p1, p2) => Self::checkers_color_at(&*p1, &*p2, p),
            Self::LinearGradient(c1, c2) => Self::linear_gradient_color_at(*c1, *c2, p),
            Self::Ring(p1, p2) => Self::ring_color_at(&*p1, &*p2, p),
            Self::Solid(c) => *c,
            Self::Stripe(p1, p2) => Self::stripe_color_at(&*p1, &*p2, p)
        }
    }

    fn checkers_color_at(p1: &Pattern, p2: &Pattern, p: Point) -> Color {
        let epsilon: f64 = 0.0001;
        let v = (p.x + epsilon).floor() + (p.y + epsilon).floor() + (p.z + epsilon).floor();

        if v as i64 % 2 == 0 {
            p1.color_at(p)
        } else {
            p2.color_at(p)
        }
    }

    fn linear_gradient_color_at(c1: Color, c2: Color, p: Point) -> Color {
        c1 + (c2 - c1) * (p.x - p.x.floor())
    }

    fn ring_color_at(p1: &Pattern, p2: &Pattern, p: Point) -> Color {
        let v = (p.x.powi(2) + p.z.powi(2)).sqrt().floor();

        if v as i64 % 2 == 0 {
            p1.color_at(p)
        } else {
            p2.color_at(p)
        }
    }

    fn stripe_color_at(p1: &Pattern, p2: &Pattern, p: Point) -> Color {
        if p.x.floor() as i64 % 2 == 0 {
            p1.color_at(p)
        } else {
            p2.color_at(p)
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Pattern {
    pattern_type: PatternType,
    transform: Matrix4x4,
    inverse_transform: Matrix4x4
}

impl Pattern {
    pub fn new(pattern_type: PatternType) -> Pattern {
        Pattern {
            pattern_type,
            transform: Matrix4x4::identity(),
            inverse_transform: Matrix4x4::identity(),
        }
    }

    pub fn checkers(p1: Pattern, p2: Pattern) -> Pattern {
        Self::new(PatternType::Checkers(Box::new(p1), Box::new(p2)))
    }

    pub fn ring(p1: Pattern, p2: Pattern) -> Pattern {
        Self::new(PatternType::Ring(Box::new(p1), Box::new(p2)))
    }

    pub fn solid(c: Color) -> Pattern {
        Self::new(PatternType::Solid(c))
    }

    pub fn stripe(c1: Color, c2: Color) -> Pattern {
        Self::new(PatternType::Stripe(Box::new(Self::solid(c1)), Box::new(Self::solid(c2))))
    }

    pub fn linear_gradient(c1: Color, c2: Color) -> Pattern {
        Self::new(PatternType::LinearGradient(c1, c2))
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
