use ray_tracer::{Canvas, Color, Point};
use ray_tracer::matrix;
use std::fs::File;

fn main() {
    let mut canvas = Canvas::new(500, 500);
    let color = Color::new(1.0, 1.0, 1.0);
    let radius = 200.0;
    let twelve = Point::new(0.0, 0.0, 1.0);
    let rotation = matrix::rotation_y(std::f64::consts::PI / 6.0);
    let canvas_transform = matrix::scale(radius, 0.0, radius).translate(250.0, 0.0, 250.0);

    (0..12).fold(twelve, |point, _| {
        let render_point = canvas_transform * point;
        canvas.set(render_point.x as usize, 500 - render_point.z as usize, color);

        rotation * point
    });

    let twelve = canvas_transform * twelve;

    canvas.set(twelve.x as usize, 500 - twelve.z as usize, Color::new(1.0, 0.0, 0.0));

    println!("Writing ppm");

    let f = File::create("clock.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
