use crate::{Matrix4x4, Point, Ray, World, Canvas};

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub half_height: f64,
    pub half_width: f64,
    pub pixel_size: f64,
    pub transform: Matrix4x4,
    pub inverse_transform: Matrix4x4
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) =
            if aspect >= 1.0 {
                (half_view, half_view / aspect)
            } else {
                (half_view * aspect, half_view)
            };
        let pixel_size = half_width * 2.0 / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_height,
            half_width,
            pixel_size,
            transform: Matrix4x4::identity(),
            inverse_transform: Matrix4x4::identity()
        }
    }

    pub fn transform(self, transform: Matrix4x4) -> Camera {
        let inverse_transform = transform.inverse().unwrap();

        Camera {
            transform,
            inverse_transform,
            ..self
        }
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.inverse_transform * Point::new(world_x, world_y, -1.0);
        let origin = self.inverse_transform * Point::origin();
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize - 1 {
            for x in 0..self.hsize - 1 {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(ray);
                canvas.set(x, y, color);
            }
        }

        canvas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Matrix4x4, Point, Vector, World};
    use std::f64::consts::PI;
    use crate::matrix::view_transform;

    #[test]
    fn creating_a_camera() {
        let camera = Camera::new(160, 120, std::f64::consts::PI / 2.0);
        assert_eq!(160, camera.hsize);
        assert_eq!(120, camera.vsize);
        assert_eq!(std::f64::consts::PI / 2.0, camera.field_of_view);
        assert_eq!(Matrix4x4::identity(), camera.transform);
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::PI / 2.0);
        assert_eq!(0.01, (c.pixel_size * 100.0).round() / 100.0);
    }

    #[test]
    fn pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::PI / 2.0);
        assert_eq!(0.01, (c.pixel_size * 100.0).round() / 100.0);
    }

    #[test]
    fn ray_through_centre_of_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Point::origin(), r.origin);
        assert_eq!(Vector::new(0.0, 0.0, -1.0), r.direction);
    }

    #[test]
    fn ray_through_corner() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(Point::origin(), r.origin);
        assert_eq!(Vector::new(0.66519, 0.33259, -0.66851), r.direction);
    }

    #[test]
    fn ray_when_camera_is_transformed() {
        let t = Matrix4x4::identity().translate(0.0, -2.0, 5.0).rotation_y(PI / 4.0);
        let c = Camera::new(201, 101, PI / 2.0).transform(t);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(Point::new(0.0, 2.0, -5.0), r.origin);
        assert_eq!(Vector::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0), r.direction);
    }

    #[test]
    fn render_world() {
        let w = World::default_world();
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = view_transform(from, to, up);
        let c = Camera::new(11, 11, PI / 2.0).transform(t);

        let image = c.render(&w);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), image.get(5, 5))
    }
}
