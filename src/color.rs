#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, c: Color) -> Color {
        Color {
            r: self.r + c.r,
            g: self.g + c.g,
            b: self.b + c.b
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, c: Color) -> Color {
        Color {
            r: self.r - c.r,
            g: self.g - c.g,
            b: self.b - c.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn color(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    #[test]
    fn create_a_color() {
        let color = color(-1.0, 4.5, 0.5);
        assert_eq!(color.r, -1.0);
        assert_eq!(color.g, 4.5);
        assert_eq!(color.b, 0.5);
    }

    #[test]
    fn adding_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(color(1.6, 0.7, 1.0), c1 + c2)
    }

    #[test]
    fn subtracting_colors() {
        let c1 = color(0.95, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        assert_eq!(color(0.25, 0.5, 0.5), c1 - c2)
    }
}