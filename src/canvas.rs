use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas {
            width: w,
            height: h,
            pixels: vec![Color::black(); w * h]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, c: Color) {
        self.pixels[y * self.width + x] = c
    }
}

impl IntoIterator for Canvas {
    type Item = Color;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(20, 10);
        assert_eq!(20, c.width);
        assert_eq!(10, c.height);

        let pixels: Vec<Color> = c.into_iter().collect();
        let expected = vec![Color::black(); 200];
        assert_eq!(expected, pixels)
    }

    #[test]
    fn setting_a_pixel_value() {
        let mut c = Canvas::new(20, 10);
        c.set(3, 1, Color::new(1.0, 1.0, 1.0));
        assert_eq!(Color::new(1.0, 1.0, 1.0), c.get(3, 1))
    }

    #[test]
    fn setting_a_pixel_value_on_the_edges() {
        let mut c = Canvas::new(20, 10);
        c.set(0, 0, Color::new(1.0, 0.5, 1.0));
        c.set(19, 9, Color::new(1.0, 1.0, 1.0));

        assert_eq!(Color::new(1.0, 0.5, 1.0), c.get(0, 0));
        assert_eq!(Color::new(1.0, 1.0, 1.0), c.get(19, 9))
    }

    #[test]
    #[should_panic]
    fn setting_a_pixel_value_out_of_bounds() {
        let mut c = Canvas::new(20, 10);
        c.set(20, 9, Color::new(1.0, 1.0, 1.0));
    }
}
