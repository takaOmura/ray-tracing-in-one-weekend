use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::vec3::*;
use std::io::Write;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
        Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn render(&mut self, world: HittableList) {
        self.initialize();

        // Render
        let mut file = std::fs::File::create("test.ppm").expect("create failed");
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("write failed");
        for j in 0..(self.image_height) as i32 {
            // if j % 10 == 0 {
            //     println!("line remaining: {}", j);
            // }
            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel00_loc
                    + self.pixel_delta_u * i as f64
                    + self.pixel_delta_v * j as f64;
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    dir: ray_direction,
                };
                // if j % 100 == 0 && i == 0 {
                //     println!("ray_direction {:?}", ray_direction);
                // }
                let pixel_color: Color = self.ray_color(ray, &world, &|| {
                    (j == 0 && i == 0) || j % 100 == 0 && i == 0
                });
                Vec3::write_color(&pixel_color, &file);
            }
        }
    }

    pub fn ray_color(&self, r: Ray, world: &HittableList, _f: &dyn Fn() -> bool) -> Color {
        let mut rec = HitRecord {
            point: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        };
        if world.hit(&r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return Vec3(
                rec.normal.x() + 1.0,
                rec.normal.y() + 1.0,
                rec.normal.z() + 1.0,
            ) * 0.5;
        }
        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        // function to print debug info, for every 100th line and first pixel of the line
        // if f() {
        //     println!("unit_direction: {:?}", unit_direction);
        //     println!("a: {:?}", a);
        // }
        Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * (a)
    }

    fn initialize(&mut self) {
        self.center = Vec3(0.0, 0.0, 0.0);
        self.image_height = || -> i32 {
            let image_height = self.image_width / self.aspect_ratio as i32;
            if image_height < 1 {
                1
            } else {
                image_height
            }
        }();
        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.image_width as f64 / self.image_height as f64;
        let focal_length = 1.0;
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -1.0 * viewport_height, 0.0);
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
        let upper_left_corner =
            self.center - viewport_u / 2.0 - viewport_v / 2.0 - Vec3(0.0, 0.0, focal_length);
        self.pixel00_loc = upper_left_corner + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0;
    }
}
