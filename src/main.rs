mod camera;
mod geometry;
mod material;
mod math;
mod ray;
mod shapes;
use crate::camera::Camera;
use geometry::Point3;
use material::Color;
use material::Lambertian;
use material::Metal;
use material::Dielectric;
use shapes::Shape;
use shapes::Sphere;
use shapes::World;
use std::rc::Rc;
use std::time::Instant;

fn main() {
    // Image dimension calculations (width fixed)
    let aspect_ratio: f64 = 8.0 / 5.0;
    let image_width = 400;
    let samples = 40;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples, max_depth);

    // Define world:
    let mut world = World::new();
    initialize_materials(&mut world);
    add_objects(&mut world);

    // Render with timer
    let start = Instant::now();
    match camera.render(&world) {
        Ok(()) => {
            println!("Finished rendering in {:?}", start.elapsed());
        }
        Err(e) => {
            println!("Failed to render scene: {e}");
        }
    }
}

fn initialize_materials(world: &mut World) {
    let ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };
    let center = Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };
    let left = Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
        fuzz: 0.3
    };
    let right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0
    };
    let glass = Dielectric {
        refraction_index: 1.5
    };
    world.materials.push(Rc::new(ground));
    world.materials.push(Rc::new(center));
    world.materials.push(Rc::new(left));
    world.materials.push(Rc::new(right));
    world.materials.push(Rc::new(glass));

}

fn add_objects(world: &mut World) {
    let sphere_center = Point3::new(0.0, 0.0, -1.2);
    let radius = 0.5;
    let sphere = Sphere {
        label: String::from("Sph1"),
        center: sphere_center,
        radius,
    };
    world.objects.push((Shape::Sphere(sphere), 2));
    world.objects.push((Shape::Sphere(Sphere {
        label: String::from("ground"),
        center: Point3::new(0.0, -100.5, 0.0),
        radius: 100.0,
    }), 1));
    world.objects.push((Shape::Sphere(Sphere {
        label: String::from("left"),
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
    }), 5));
    world.objects.push((Shape::Sphere(Sphere {
        label: String::from("right"),
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
    }), 4));
}
