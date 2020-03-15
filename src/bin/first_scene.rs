use ray_tracer::{Camera, Color, Matrix4x4, Shape, Point, PointLight, Material, Vector, World};
use std::fs::File;
use std::f64::consts::PI;
use ray_tracer::matrix::view_transform;

fn main() {
    let room_material = Material::new().specular(0.0).color(Color::new(1.0, 0.9, 0.9));

    let floor = Shape::sphere().
        transform(Matrix4x4::identity().scale(10.0, 0.01, 10.0)).
        material(room_material.clone());
    let left_wall = Shape::sphere().
        material(room_material.clone()).
        transform(Matrix4x4::identity().scale(10.0, 0.01, 10.0).rotation_x(PI / 2.0).rotation_y(-PI/4.0).translate(0.0, 0.0, 5.0));
    let right_wall = Shape::sphere().
        material(room_material.clone()).
        transform(Matrix4x4::identity().scale(10.0, 0.01, 10.0).rotation_x(PI / 2.0).rotation_y(PI/4.0).translate(0.0, 0.0, 5.0));

    let left = Shape::sphere().
        material(Material::new().color(Color::new(1.0, 0.8, 0.1)).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.33, 0.33, 0.33).translate(-1.5, 1.5, -0.75));

    let middle = Shape::sphere().
        material(Material::new().color(Color::new(1.0, 0.0, 1.0)).diffuse(0.7).specular(0.8).ambient(0.2)).
        transform(Matrix4x4::identity().translate(-0.5, 1.0, 0.5));

    let right = Shape::sphere().
        material(Material::new().color(Color::new(0.5, 1.0, 0.1)).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.5, 0.5, 0.5).translate(1.5, 0.5, -0.5));

    let world = World {
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white()),
        objects: vec![floor, left_wall, right_wall, left, middle, right]
    };

    let view = view_transform(Point::new(0.0, 1.5, -5.0), Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let camera = Camera::new(600, 300, PI / 3.0).transform(view);
    let canvas = camera.render(&world);

    println!("Writing ppm");
    let f = File::create("first_scene.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
