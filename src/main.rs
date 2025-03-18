mod geometry;
mod ray;
mod shapes;
mod math;
mod camera;

use shapes::Shape;
use shapes::Sphere;
use geometry::Point3;
use shapes::World;
use crate::camera::Camera;
use std::io::{self, BufWriter, Write};
use std::time::Instant;


fn main() {
    // Image dimension calculations (width fixed)
    let aspect_ratio: f64 = 8.0 / 5.0;
    let image_width = 3024;
    let camera = Camera::new(aspect_ratio, image_width);
    
    // Define world:
    let mut world = World {
        objects: vec![]
    };
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let sphere = Sphere {center: sphere_center, radius};
    world.objects.push(Shape::Sphere(sphere));
    world.objects.push(Shape::Sphere(Sphere {center: Point3::new(0.0, -100.5, 0.0), radius: 100.0}));

    // Timer start
    let start = Instant::now();
    match camera.render(&world) {
        Ok(()) => {
            println!("Finished rendering in {:?}", start.elapsed());
        },
        Err(e) => {
            println!("Failed to render scene: {e}");
        }
    }
}

