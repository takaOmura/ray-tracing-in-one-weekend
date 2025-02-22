use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
}
impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t_min: f64, ray_t_max: f64, rec: &mut HitRecord) -> bool {
        let center = self.center;
        let radius = self.radius;
        let oc = center - ray.origin;
        let a = ray.dir.length_squared();
        let h = ray.dir.dot(oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if root <= ray_t_min || ray_t_max <= root {
            root = (h + sqrtd) / a;
            if root <= ray_t_min || ray_t_max <= root {
                return false;
            }
        }
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - center) / radius;
        rec.set_face_normal(ray, outward_normal);
        true
    }
}
