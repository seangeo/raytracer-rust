use ray_tracer::{Canvas, Color, Shape, Point, PointLight, Ray, intersection, Material};
use std::fs::File;

fn main() {
    let ray_orign = Point::new(0.0, 0.0, -15.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 400;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let material = Material::new().color(Color::new(1.0, 0.8, 1.0));
    let sphere = Shape::sphere().material(material);
    let light = PointLight::new(Point::new(-1.0, 10.0, -10.0), Color::new(1.0, 1.0, 2.0));

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_orign, (position - ray_orign).normalize());
            let xs = sphere.intersects(&ray);

            match intersection::hit(&xs) {
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let eye = -ray.direction;
                    let color = hit.object.material.lighting(light, point, eye, normal);

                    canvas.set(x, y, color)
                }
                None => ()
            }
        }
    }

    println!("Writing ppm");

    let f = File::create("light_caster.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
