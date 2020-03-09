use crate::Matrix4x4;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    pub half_height: f64,
    pub half_width: f64,
    pub pixel_size: f64,
    pub transform: Matrix4x4
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
            transform: Matrix4x4::identity()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Matrix4x4;

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
}
