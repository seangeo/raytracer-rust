use ray_tracer::{Canvas, Color, Point, Vector};
use std::fs::File;

fn tick(canvas: & mut Canvas, (g, w) : (Vector, Vector), (p, v) : (Point, Vector)) {
   // println!("position: {:?}, velocity: {:?}", p, v);

    if p.y < 550.0 && p.y >= 0.0 {
        canvas.set(p.x as usize, 550 - (p.y as usize), Color::new(1.0, 0.0, 0.0));
    }

    if p.y > 0.0 {
        let p = p + v;
        let v = v + g + w;
        tick(canvas, (g, w), (p, v));
    } else {
        println!("Complete")
    }
}

fn main() {
    let mut canvas = Canvas::new(900, 550);
    let environment = (Vector::new(0.0, -0.1, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let projectile = (Point::new(0.0, 1.0, 0.0), Vector::new(0.2, 1.8, 0.0).normalize() * 11.25);

    tick(&mut canvas, environment, projectile);

    println!("Writing ppm");

    let f = File::create("cannon.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
