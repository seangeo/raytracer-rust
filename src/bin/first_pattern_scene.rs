use ray_tracer::{Camera, Color, Matrix4x4, Shape, Pattern, Point, PointLight, Material, Vector, World};
use std::fs::File;
use std::f64::consts::PI;
use ray_tracer::matrix::view_transform;

fn main() {
    let pattern = Pattern::stripe(Color::new(0.9, 0.1, 0.8), Color::white());
    let room_material = Material::new().specular(0.0).pattern(pattern);

    let floor = Shape::plane().
        //transform(Matrix4x4::identity().scale(10.0, 0.01, 10.0)).
        material(room_material);

    let backdrop = Shape::plane().
        transform(Matrix4x4::identity().rotation_x(PI /  2.0).translate(0.0, 0.0, 10.0)).
        material(room_material.diffuse(0.7).specular(0.1));

    let left = Shape::sphere().
        material(Material::new().pattern(pattern).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.33, 0.33, 0.33).translate(-2.0, 1.35, -0.75));

    let middle = Shape::sphere().
        material(Material::new().pattern(pattern).diffuse(0.7).specular(0.8).ambient(0.2)).
        transform(Matrix4x4::identity().translate(-0.5, 1.0, 0.5));

    let right = Shape::sphere().
        material(Material::new().pattern(pattern).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.5, 0.5, 0.5).translate(1.8, -0.125, -0.5));

    let world = World {
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white()),
        objects: vec![floor, backdrop,  left, middle, right]
    };

    let view = view_transform(Point::new(-2.0, 1.5, -5.0), Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let camera = Camera::new(600, 400, PI / 3.0).transform(view);
    let canvas = camera.render(&world);

    println!("Writing ppm");
    let f = File::create("first_pattern_scene.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}