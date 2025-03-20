use rand::rngs::ThreadRng;

// material.rs
use crate::geometry::Vec3;
use crate::ray::Ray;
use crate::shapes::HitRecord;
use std::fmt::Debug;
pub type Color = Vec3;
pub const BLUE: Color = Color {
    x: 0.5,
    y: 0.7,
    z: 1.0,
};
pub const WHITE: Color = Color {
    x: 1.0,
    y: 1.0,
    z: 1.0,
};
pub const BLACK: Color = Color {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const RED: Color = Color {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const GREEN: Color = Color {
    x: 50.0 / 255.0,
    y: 200.0 / 255.0,
    z: 90.0 / 255.0,
};

pub trait Material: Debug {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, rng: &mut ThreadRng) -> (Color, Ray);
}

#[derive(Debug)]
pub struct DefaultMaterial {}

impl Material for DefaultMaterial {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, rng: &mut ThreadRng) -> (Color, Ray) {
        panic!("No material assigned for {:?}", hit_rec)
    }
}

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, rng: &mut ThreadRng) -> (Color, Ray) {
        let mut scatter_direction = hit_rec.normal + Vec3::sample_unit_vector(rng);
        if Vec3::too_small(scatter_direction) {
            scatter_direction = hit_rec.normal;
        }
        let out_ray = Ray {
            direction: scatter_direction,
            origin: hit_rec.p,
        };
        (self.albedo, out_ray)
    }
}

#[derive(Debug)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_rec: &HitRecord, rng: &mut ThreadRng) -> (Color, Ray) {
        (
            self.albedo,
            Ray {
                origin: hit_rec.p,
                direction: Vec3::reflect(ray_in.direction, hit_rec.normal),
            },
        )
    }
}
