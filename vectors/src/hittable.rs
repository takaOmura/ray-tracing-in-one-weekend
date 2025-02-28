use crate::interval::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            material: Material::Lambertian(Color::new(0.0, 0.0, 0.0)),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub enum HittableEnum {
    Sphere(Sphere),
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

impl Hittable for HittableEnum {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableEnum::Sphere(sphere) => sphere.hit(ray, ray_t, rec),
        }
    }
}

pub struct HittableList {
    pub objects: Vec<HittableEnum>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            point: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Material::None,
        };

        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            let interval = Interval::new(ray_t.min, closest_so_far);
            if object.hit(ray, interval, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.point = temp_rec.point;
                rec.normal = temp_rec.normal;
                rec.material = temp_rec.material;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }
        hit_anything
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }
        rec.t = root;
        rec.point = ray.at(rec.t);
        let outward_normal = (rec.point - center) / radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = self.material;
        true
    }
}
