use crate::{Point, Vector, Color, PointLight};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::white(),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0
        }
    }

    pub fn ambient(self, ambient: f64) -> Material {
        Material {
            ambient,
            ..self
        }
    }

    pub fn color(self, color: Color) -> Material {
        Material {
            color,
            ..self
        }
    }

    pub fn diffuse(self, diffuse: f64) -> Material {
        Material {
            diffuse,
            ..self
        }
    }

    pub fn specular(self, specular: f64) -> Material {
        Material {
            specular,
            ..self
        }
    }

    pub fn shininess(self, shininess: f64) -> Material {
        Material {
            shininess,
            ..self
        }
    }

    pub fn lighting(&self, light: PointLight, position: Point, eye: Vector, normal: Vector, in_shadow: bool) -> Color {
        let effective_color = self.color * light.intensity;
        let ambient = effective_color * self.ambient;
        let mut diffuse = Color::black();
        let mut specular = Color::black();

        if !in_shadow {
            let lightv = (light.position - position).normalize();
            let light_dot_normal = lightv.dot(normal);

            if light_dot_normal >= 0.0 {
                diffuse = effective_color * self.diffuse * light_dot_normal;

                let reflectv = -lightv.reflect(normal);
                let reflect_dot_eye = reflectv.dot(eye);

                if reflect_dot_eye > 0.0 {
                    let factor = reflect_dot_eye.powf(self.shininess);
                    specular = light.intensity * self.specular * factor;
                }
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Vector, Color, PointLight};

    #[test]
    fn eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
        let result = Color::new(1.9, 1.9, 1.9);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, false))
    }

    #[test]
    fn eye_between_light_and_surface_offset_45_deg() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
        let result = Color::new(1.0, 1.0, 1.0);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, false))
    }

    #[test]
    fn eye_between_light_and_surface_light_offset_45_deg() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
        let result = Color::new(0.7364, 0.7364, 0.7364);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, false))
    }

    #[test]
    fn eye_in_path_of_light() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, -(2_f64.sqrt() / 2.0), -(2_f64.sqrt() / 2.0));
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::white());
        let result = Color::new(1.6364, 1.6364, 1.6364);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, false))
    }

    #[test]
    fn light_behind_the_surface() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::white());
        let result = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, false))
    }

    #[test]
    fn light_for_surface_in_shadow() {
        let m = Material::new();
        let position = Point::origin();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::white());
        let result = Color::new(0.1, 0.1, 0.1);

        assert_eq!(result, m.lighting(light, position, eyev, normalv, true));
    }
}
