// shapes.rs
// Defines primitive shapes and their geometry
use crate::geometry::{Point3, Vec3};
use crate::ray::Ray;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Ourward normal must have unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
    pub fn new() -> Self {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        let c_min_p: Vec3 = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let h = ray.direction.dot(c_min_p);
        let c = c_min_p.dot(c_min_p) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant <= 0.0 {
            false
        } else {
            let roots: [f64; 2] = [(h - discriminant.sqrt()) / a, (h + discriminant.sqrt()) / a];
            let intersect: f64;
            // Get first intersection in range
            if t_min <= roots[0] && roots[0] <= t_max {
                intersect = roots[0];
            } else if t_min <= roots[1] && roots[1] <= t_max {
                intersect = roots[1];
            } else {
                return false;
            }
            hit_rec.p = ray.at(intersect);
            hit_rec.t = intersect;
            let outward_normal = (hit_rec.p - self.center) / self.radius;
            hit_rec.set_face_normal(ray, outward_normal); // clunky
            true
        }
    }
}

pub enum Shape {
    Sphere(Sphere)
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        match self {
            Shape::Sphere(s) => s.hit(ray, t_min, t_max, hit_rec),
            _ => false
        }
    }
}

pub struct World {
    pub objects: Vec<Shape>
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        //let mut temp_rec = HitRecord::new();
        let mut closest_t = t_max;
        let mut hit_anything = false;
        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_t, hit_rec) {
                hit_anything = true;
                closest_t = hit_rec.t;
            }
        }
        hit_anything
    }
}