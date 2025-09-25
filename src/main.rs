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
use material::{GREEN, WHITE, PINK, RED, BLUE};
use shapes::Shape;
use shapes::Sphere;
use shapes::World;
use std::f64::consts::PI;
use std::sync::Arc;
use std::time::Instant;

/*
Rendering 'Shiny Metal' scene at 400x250, max_depth = 50 took 710s
# Samples:          10          1000
Without Rayon:      6.8s        710s
Rayon:              1.1s        104.4s

*/

fn main() {
    // Image dimension calculations (width fixed)
    let aspect_ratio: f64 = 8.0 / 5.0;
    let image_width = 400;
    let location = Point3::new(7.0,4.0,7.0);
    let view_target = Point3::new(0.0,3.0,0.1);
    let vfov = 46.0;
    let focal_length = 10.0;
    let focal_angle = 0.5; 
    let samples = 40;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, location, view_target, focal_length, focal_angle, vfov, samples, max_depth);

    // Define world:
    let mut world = World::new();

    //initialize_materials(&mut world);
    //add_objects(&mut world);
    make_scene(&mut world);

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
        fuzz: 0.0
    };
    let right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0
    };
    let glass = Dielectric {
        refraction_index: 1.5
    };
    let air_in_water =  Dielectric {
        refraction_index: 1.0/1.33
    };
    let air_in_glass = Dielectric {
        refraction_index: 1.0 / 1.5
    };
    world.materials.push(Arc::new(ground));
    world.materials.push(Arc::new(center));
    world.materials.push(Arc::new(left));
    world.materials.push(Arc::new(right));
    world.materials.push(Arc::new(glass));
    world.materials.push(Arc::new(air_in_water));
    world.materials.push(Arc::new(air_in_glass));

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
    }), 3));
    world.objects.push((Shape::Sphere(Sphere {
        label: String::from("right"),
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
    }), 5));
    
    world.objects.push((Shape::Sphere(Sphere {
        label: String::from("interior"),
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.1,
    }), 3));
    
}

fn make_scene(world: &mut World) {
    let rng = rand::thread_rng();
    let ground = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    let glass = Dielectric {
        refraction_index: 1.5
    };
    let metal = Metal {
        albedo: Color::new(0.9,0.9,0.9),
        fuzz: 0.0
    };
    world.add_material(ground);
    world.add_material(glass);
    world.add_material(metal);

    world.add_material(Lambertian {albedo: GREEN});
    world.add_material(Lambertian {albedo: RED});
    world.add_material(Lambertian {albedo: PINK});

    /* 
    let ground_sphere = Sphere::new(0.0, -1000.0, 0.0, 1000.0);
    world.objects.push((Shape::Sphere(ground_sphere), 1));

    let central_sphere = Sphere::new(0.0, 1.0, 0.0, 1.0);
    world.objects.push((Shape::Sphere(central_sphere), 5));
    
    let s1 = Sphere::new(1.7, 0.7, 0.0, 0.7);
    world.objects.push((Shape::Sphere(s1), 2));

    let s2 = Sphere::new(2.8, 0.4, 0.0, 0.4);
    world.objects.push((Shape::Sphere(s2), 4));

    let s3 = Sphere::new(-5.0, 4.0, 0.0, 4.0);
    world.objects.push((Shape::Sphere(s3), 3));

    let big_sphere = Sphere::new(2.0, 10.0, -13.0, 10.0);
    world.objects.push((Shape::Sphere(big_sphere), 3));

    let big_sphere = Sphere::new(100.0, 100.0, 30.0, 100.0);
    world.objects.push((Shape::Sphere(big_sphere), 6));
    */
    let ground_sphere = Sphere::new(0.0, -1000.0, 0.0, 1000.0);
    world.objects.push((Shape::Sphere(ground_sphere), 1));

    let central_sphere = Sphere::new(0.0, 1.0, 0.0, 1.0);
    world.objects.push((Shape::Sphere(central_sphere), 5));
}