use ray_tracer::{Camera, Color, Matrix4x4, Shape, Pattern, Point, PointLight, Material, Vector, World};
use std::fs::File;
use std::f64::consts::PI;
use ray_tracer::matrix::view_transform;

fn main() {
    let t = Matrix4x4::identity().scale(0.1, 0.1, 0.1).rotation_z(PI / 4.0);
    let object_pattern = Pattern::stripe(Color::new(1.0, 0.0, 1.0), Color::white()).transform(t);

    let t = Matrix4x4::identity().scale(13.0, 1.0, 1.0).rotation_y(-PI / 2.0).translate(0.0, 0.0, 10.0);
    let floor_pattern = Pattern::linear_gradient(Color::white(), Color::black()).transform(t);
    let t = Matrix4x4::identity().scale(0.5, 0.5, 0.5).rotation_z(PI / 1.0);
    let wall_pattern = Pattern::checkers(Pattern::solid(Color::new(0.1, 0.1, 0.5)), Pattern::solid(Color::white())).transform(t);
    let floor_material = Material::new().specular(0.0).pattern(floor_pattern);
    let wall_material = Material::new().specular(0.0).pattern(wall_pattern).diffuse(0.7);

    let floor = Shape::plane().material(floor_material);

    let backdrop = Shape::plane().
        transform(Matrix4x4::identity().rotation_x(PI /  2.0).translate(0.0, 0.0, 10.0)).
        material(wall_material);


    let left_p_t = Matrix4x4::identity().scale(2.0, 1.0, 1.0).rotation_z(-PI / 2.0).translate(0.0, -1.0, 0.0);
    let left_pattern = Pattern::linear_gradient(Color::new(1.0, 1.0, 0.0), Color::new(0.0, 1.0, 0.0)).transform(left_p_t);
    let left = Shape::sphere().
        material(Material::new().pattern(left_pattern).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.33, 1.0, 0.33).translate(-2.0, 1.35, -0.75));

    let middle = Shape::sphere().
        material(Material::new().pattern(object_pattern.clone()).diffuse(0.7).specular(0.8).ambient(0.2)).
        transform(Matrix4x4::identity().rotation_y(PI / 2.0).translate(-0.5, 1.0, 0.5));

    let right_pattern = Pattern::ring(Pattern::solid(Color::new(1.0, 0.0, 0.0)), Pattern::solid(Color::new(0.0, 1.0, 0.0)));
    let right_pattern = right_pattern.transform(Matrix4x4::identity().scale(0.125, 1.0, 0.125).rotation_x(PI / 2.0));
    let right = Shape::sphere().
        material(Material::new().pattern(right_pattern).diffuse(0.7).specular(0.3)).
        transform(Matrix4x4::identity().scale(0.5, 0.5, 0.5).translate(1.8, 1.0, -0.5));

    let world = World {
        light: PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white()),
        objects: vec![floor, backdrop, left, middle, right]
    };

    let view = view_transform(Point::new(0.0, 1.5, -5.0), Point::new(0.0, 1.0, 0.0), Vector::new(0.0, 1.0, 0.0));
    let camera = Camera::new(600, 400, PI / 3.0).transform(view);
    let canvas = camera.render(&world);

    println!("Writing ppm");
    let f = File::create("first_pattern_scene.ppm").unwrap();
    let mut f = std::io::BufWriter::new(f);
    canvas.to_ppm(& mut f).unwrap();
}
