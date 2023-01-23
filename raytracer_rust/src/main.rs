mod vec3;
use vec3::{Vec3, length, normalize, Color, random_in_unit_sphere};

mod ray;
use ray::Ray;

mod hittable;
use hittable::*;

mod world;
use world::World;




fn main() {


    let mut world = World::new(1080);
    let z = -2.0;
    let x = 0.0;
    world.add(Shape::sphere(Vec3::new(x+0.2, 0.4, z-1.0), 0.3), Material::Lambertian(Color::red()));
    world.add(Shape::sphere(Vec3::new(x+0.0, -0.5, z-3.0), 0.5), Material::Lambertian(Color::orange()));
    world.add(Shape::sphere(Vec3::new(x+0.6, -0.2, z-0.5), 0.2), Material::Lambertian(Color::blue()));
    world.add(Shape::sphere(Vec3::new(x-0.8, 0.5, z-0.4), 0.5), Material::Lambertian(Color::green()));
    world.add(Shape::plane(0.0, 1.0, 0.0, -1.2), Material::Lambertian(Color::white()));
    // dbg!(world.get_normal_at_surface_point(
    //     &Vec3::new(0.0, -0.501, -3.0)
    // ));

    world.set_sun(Shape::sphere(Vec3::new(-10.0, 8.0, 5.0), 2.0));
    // world.add(Shape::sphere(Vec3::new(-0.5, 0.1, -4.0), 0.5), Material::Lambertian(Color::blue()));
    // world.add(Shape::sphere(Vec3::new(2.0, 17.0, 0.0), 5.0), Material::Light);
    // world.add(Shape::plane(0.0, 1.0, 0.0, -8.0), Material::Light);
    // world.add(Shape::sphere(Vec3::new(-1.0, 1.0, 0.0), 0.1), Material::Light);
    world.render();


}

