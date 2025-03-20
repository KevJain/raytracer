// shapes.rs
// Defines primitive shapes and their geometry
use crate::geometry::{Point3, Vec3};
use crate::ray::Ray;
use crate::math::Interval;
#[derive(Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // Outward normal must have unit length
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal).is_sign_negative();
        if self.front_face {
            self.normal = outward_normal
        } else {
            println!("Inside sphere!");
            println!("{:?}", ray);
            println!("{:?}", ray.direction.dot(outward_normal));
            println!("{:?}", outward_normal.len());
            println!("{:?}\n", outward_normal);
            //println!("{:?}", outward_normal);
            self.normal = -outward_normal
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
    fn hit(&self, ray: &Ray, time: &Interval, hit_rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub label : String,
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time: &Interval, hit_rec: &mut HitRecord) -> bool {
        let c_min_p: Vec3 = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let h = ray.direction.dot(c_min_p);
        let c = c_min_p.dot(c_min_p) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant.is_sign_negative() {
            false
        } else {
            let sqrtd = discriminant.sqrt();
            let roots: [f64; 2] = [(h - sqrtd) / a, (h + sqrtd) / a];
            let intersect: f64;
            // Get first intersection in range
            if time.contains(roots[0]) {
                intersect = roots[0];
            } else if time.contains(roots[1]) {
                intersect = roots[1];
            } else {
                return false;
            }
            hit_rec.p = ray.at(intersect);
            hit_rec.t = intersect;
            //hit_rec.normal = (hit_rec.p - self.center) / self.radius; // Remove
            let outward_normal = (hit_rec.p - self.center) / self.radius;
            hit_rec.set_face_normal(ray, outward_normal); // clunky
            //println!("Hit {}, {:?}", self.label, hit_rec);
            true
        }
    }
}

pub enum Shape {
    Sphere(Sphere)
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, time: &Interval, hit_rec: &mut HitRecord) -> bool {
        match self {
            Shape::Sphere(s) => s.hit(ray, time, hit_rec),
        }
    }
}

pub struct World {
    pub objects: Vec<Shape>
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, time: &Interval, hit_rec: &mut HitRecord) -> bool {
        //let mut temp_rec = HitRecord::new();
        let mut closest_t = time.max;
        let mut hit_anything = false;
        for object in self.objects.iter() {
            if object.hit(ray, &Interval::new(time.min, closest_t), hit_rec) {
                hit_anything = true;
                closest_t = hit_rec.t;
            }
        }
        hit_anything
    }
}