use ray_tracer::{Point, Vector};

fn tick((g, w) : (Vector, Vector), (p, v) : (Point, Vector)) {
    println!("position: {:?}, velocity: {:?}", p, v);

    if p.y > 0.0 {
        let p = p + v;
        let v = v + g + w;
        tick((g, w), (p, v));
    } else {
        println!("Complete")
    }
}

fn main() {
    let environment = (Vector::new(0.0, -0.5, 0.0), Vector::new(-0.01, 0.0, 0.0));
    let projectile = (Point::new(0.0, 1.0, 0.0), Vector::n(1, 1, 0).normalize());

    tick(environment, projectile);
}
