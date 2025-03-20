mod camera;
mod geometry;
mod math;
mod ray;
mod shapes;

use crate::camera::Camera;
use geometry::Point3;
use shapes::Shape;
use shapes::Sphere;
use shapes::World;
use std::time::Instant;

fn main() {
    // Image dimension calculations (width fixed)
    let aspect_ratio: f64 = 8.0 / 5.0;
    let image_width = 400;
    let samples = 20;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples, max_depth);

    // Define world:
    let mut world = World { objects: vec![] };
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let sphere = Sphere {
        label: String::from("Sph1"),
        center: sphere_center, 
        radius,
    };
    world.objects.push(Shape::Sphere(sphere));
    world.objects.push(Shape::Sphere(Sphere {
        label: String::from("Sph2"),
        center: Point3::new(0.0, -100.5, 0.0),
        radius: 100.0,
    }));

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
