// shapes.rs
// Defines primitive shapes and their geometry
use crate::geometry::{Point3, Vec3};
use crate::ray::Ray;
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let c_min_p: Vec3 = self.center - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let h = ray.direction.dot(c_min_p);
        let c = c_min_p.dot(c_min_p) - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant <= 0.0 {
            None
        } else {
            let roots: [f64; 2] = [(h - discriminant.sqrt()) / a, (h + discriminant.sqrt()) / a];
            let intersect: f64;
            // Get first intersection in range
            if t_min <= roots[0] && roots[0] <= t_max {
                intersect = roots[0];
            } else if t_min <= roots[1] && roots[1] <= t_max {
                intersect = roots[1];
            } else {
                return None;
            }
            let intersect_point = ray.at(intersect);
            let normal = (intersect_point - self.center) / self.radius;
            Some(HitRecord {p: intersect_point, normal: normal, t: intersect})
        }
    }
}
