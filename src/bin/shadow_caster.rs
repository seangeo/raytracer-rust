use ray_tracer::{Canvas, Color, Shape, Point, Ray, intersection};
use std::fs::File;

fn main() {
    let ray_orign = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 300;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let sphere = Shape::sphere();

    for y in 0..canvas_pixels - 1 {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels - 1 {
            let world_x = -half + pixel_size * x as f64;
            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_orign, (position - ray_orign).normalize());
            let xs = sphere.intersects(ray);

            match intersection::hit(&xs) {
                Some(_) => canvas.set(x, y, color),
                None => ()
            }
        }
    }

    println!("Writing ppm");

    let f = File::create("shadow_caster.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
