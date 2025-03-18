mod color;
mod geometry;
mod ray;

use color::Color;
use color::write_pixel;
use geometry::Point3;
use geometry::Vec3;
use ray::Ray;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};
const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0
};

fn main() {
    // Image dimension calculations (width fixed)
    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width = 3024;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let camera_center: Vec3 = Point3::new(0.0, 0.0, 0.0);
    let focal_length: f64 = 1.0;

    // Viewport calculations based on aspect ratio, viewport_height is arbitrary scaling param
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

    // Directions are x: right, y: up, z: forward
    // Since the PPM format is written top left to bottom right, we need to remap coordinates
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Invert y-direction
    // Now u still points rightwards, but v points downwards!

    // Get distance between pixel centers:
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location relative to the camera of the upper left point of the viewport
    let viewport_upper_left: Point3 =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;

    // New origin for the viewport coordinates: offset by half of pixel gap to get pixel center
    let pixel00: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

    let start = Instant::now();
    match render(
        image_width,
        image_height,
        pixel00,
        pixel_delta_u,
        pixel_delta_v,
        camera_center,
    ) {
        Ok(()) => {
            let duration = start.elapsed();
            println!("Rendered file successfully. Time taken: {:?}", duration);
        }
        Err(e) => println!("Failed to render file. Error: {e}"),
    }
}

fn ray_color(ray: &Ray) -> Color {
    if ray_sphere_intersection(ray, Point3::new(0.0,0.0,1.0), 0.5) {
        return BLACK;
    }
    let unit_direction = ray.direction.normalize();
    let a = (unit_direction.y + 1.0) * 0.5;
    (a) * BLUE + (1.0 - a) * WHITE
}

fn ray_sphere_intersection(ray: &Ray, center: Point3, radius: f64) -> bool {
    let c_min_p: Vec3 = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * (ray.direction.dot(c_min_p));
    let c = c_min_p.dot(c_min_p) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

// Outputs the image to output.ppm
fn render(
    width: u32,
    height: u32,
    pixel00: Point3,
    delta_u: Vec3,
    delta_v: Vec3,
    camera_center: Point3,
) -> io::Result<()> {
    let file = File::create("output.ppm")?;
    let mut buf_writer = BufWriter::new(file);
    writeln!(buf_writer, "P3")?;
    writeln!(buf_writer, "{} {}", width, height)?;
    writeln!(buf_writer, "255")?;

    for row in 0..(height) {
        io::stdout().flush()?;
        print!("\rRendering line {}", row);
        for col in 0..(width) {
            let pixel_center = pixel00 + delta_u * (col as f64) + delta_v * (row as f64);
            let ray_direction = pixel_center - camera_center;

            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };
            let color = ray_color(&ray);
            write_pixel(&mut buf_writer, color)?;
        }
    }
    println!("");

    Ok(())
}
