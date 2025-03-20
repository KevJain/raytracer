// camera.rs
// Controls camera data and position

use crate::geometry::{Point3, Vec3};
use crate::math::Interval;
use crate::ray::Ray;
use crate::shapes::{HitRecord, Hittable, World};
use rand::Rng;
use rand::rngs::ThreadRng;
use std::fs::File;
use std::io::{self, BufWriter, Result, Write};
use crate::material::Color;
use crate::material::GREEN;
use crate::material::BLUE;
use crate::material::WHITE;
use crate::material::BLACK;
use crate::material::RED;


pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,
    pub location: Point3,
    pub samples: u32,
    pub max_depth: i32,
    // Viewport fields:
    pixel00: Point3,
    delta_u: Vec3,
    delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples: u32, max_depth: i32) -> Self {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        let focal_length: f64 = 1.0;
        let origin = Point3::new(0.0, 0.0, 0.0);

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
            origin - Vec3::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;

        // New origin for the viewport coordinates: offset by half of pixel gap to get pixel center
        let pixel00: Point3 = viewport_upper_left + (pixel_delta_u + pixel_delta_v) / 2.0;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            location: origin,
            samples,
            max_depth,
            pixel00,
            delta_u: pixel_delta_u,
            delta_v: pixel_delta_v,
        }
    }

    pub fn render(&self, world: &World) -> io::Result<()> {
        println!("Writing {} x {} image", self.image_width, self.image_height);
        let file = File::create("output.ppm")?;
        let mut buf_writer = BufWriter::new(file);
        writeln!(buf_writer, "P3")?;
        writeln!(buf_writer, "{} {}", self.image_width, self.image_height)?;
        writeln!(buf_writer, "255")?;

        let mut rng: ThreadRng = rand::thread_rng();
        for row in 0..(self.image_height) {
            io::stdout().flush()?;
            print!("\rRendering line {}", row);
            for col in 0..(self.image_width) {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for i in 0..self.samples {
                    //println!("Casting Ray at ({}, {})", row, col);
                    let ray = self.get_ray(row, col, &mut rng);
                    color = color + Camera::ray_color(&ray, world, &mut rng, self.max_depth);
                    //println!();
                }
                let color_avg = color / self.samples as f64;
                let gamma_corrected_color = Self::color_gamma_transform(color_avg, 2.0);
                Camera::write_pixel(&mut buf_writer, gamma_corrected_color)?;
            }
        }
        println!("");

        Ok(())
    }

    fn get_ray(&self, pixel_row: u32, pixel_col: u32, rng: &mut ThreadRng) -> Ray {
        // Pixels are located at the center of the square they occupy
        // Thus, we sample an offset in [-0.5,0.5) x [-0.5,0.5) to get a ray in the sample pixel
        // u is the change of coordinate for the x direction, and v is the change of coordinate for the y direction
        let offset_v: f64 = rng.r#gen::<f64>() - 0.5;
        let offset_u: f64 = rng.r#gen::<f64>() - 0.5;
        let viewport_location: Point3 = self.pixel00
            + ((pixel_row as f64 + offset_v) * self.delta_v)
            + ((pixel_col as f64 + offset_u) * self.delta_u);

        Ray {
            origin: self.location,
            direction: viewport_location - self.location,
        }
    }

    fn write_pixel<W: Write>(writer: &mut W, color: Color) -> Result<()> {
        let rbyte = (color.x * 255.999) as u8;
        let gbyte = (color.y * 255.999) as u8;
        let bbyte = (color.z * 255.999) as u8;
        writeln!(writer, "{} {} {}", rbyte, gbyte, bbyte)
    }

    fn color_gamma_transform(color: Color, gamma: f64) -> Color {
        Color::new(
            Self::linear_to_gamma(color.x, gamma),
            Self::linear_to_gamma(color.y, gamma),
            Self::linear_to_gamma(color.z, gamma),
        )
    }

    // Transforms linear colour space to gamma
    fn linear_to_gamma(linear: f64, gamma: f64) -> f64 {
        linear.powf(1.0 / gamma)
    }

    fn color_rgb(r: u8, g: u8, b: u8) -> Color{
        Color::new(r as f64/255.0, g as f64/255.0, b as f64/255.0)
    }

    fn ray_color(ray: &Ray, world: &World, rng: &mut ThreadRng, depth: i32) -> Color {
        if depth <= 0 {
            return RED;
        }
        let mut hit_rec = world.new_hitrecord();
        if world.hit(ray, &Interval::new(0.001, 100000000000.0), &mut hit_rec) {
            let (attenuation, new_ray) = hit_rec.material.scatter(ray, &hit_rec, rng);
            attenuation * Self::ray_color(&new_ray, world, rng, depth - 1)
        } else {
            let unit_direction = ray.direction.normalize();
            let a = (unit_direction.y + 1.0) * 0.5;
            (a) * BLUE + (1.0 - a) * WHITE
        }
    }
}
