use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::utils::*;
use crate::vec3::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    look_from: Point3,
    look_at: Point3,
    vup: Vec3,
    image_height: i32,
    center: Point3,
    pixel_sample_scale: f64,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        vfov: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
    ) -> Camera {
        Camera {
            aspect_ratio,
            vfov,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            pixel_sample_scale: 0.0,
            u: Vec3(0.0, 0.0, 0.0),
            v: Vec3(0.0, 0.0, 0.0),
            w: Vec3(0.0, 0.0, 0.0),
            look_at,
            look_from,
            vup,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn render<F: FnMut(String)>(&mut self, mut output: F, world: HittableList) {
        self.initialize();

        // Render
        output(format!(
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        ));

        for j in 0..(self.image_height) as i32 {
            for i in 0..self.image_width {
                let pixel_color = (0..self.samples_per_pixel)
                    .map(|_| {
                        let r = self.get_ray(i, j);
                        self.ray_color(r, &world, self.max_depth)
                    })
                    .fold(Vec3(0.0, 0.0, 0.0), |acc, x| acc + x)
                    * self.pixel_sample_scale as f64;

                output(Vec3::get_color(&pixel_color));
            }
        }
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let origin = self.center;
        Ray {
            origin,
            dir: self.pixel00_loc
                + self.pixel_delta_u * (i as f64 + offset.x())
                + self.pixel_delta_v * (j as f64 + offset.y())
                - origin,
        }
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    // if hits an object, return the color of the object
    // else return the background color
    fn ray_color(&self, r: Ray, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();

        if world.hit(&r, Interval::new(0.0001, f64::INFINITY), &mut rec) {
            let (ok, scattered, attenuation) =
                rec.material
                    .scatter(&r, rec.point, rec.normal, rec.front_face);
            if ok {
                return attenuation * self.ray_color(scattered, world, depth - 1);
            }
            return Color::new(1.0, 1.0, 1.0);
        }

        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3(1.0, 1.0, 1.0) * (1.0 - a) + Vec3(0.5, 0.7, 1.0) * (a)
    }

    fn initialize(&mut self) {
        self.center = self.look_from;
        self.image_height = {
            let image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
            if image_height < 1 {
                1
            } else {
                image_height
            }
        };
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        let focal_length = (self.look_at - self.look_from).length();

        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = h * 2.0 * focal_length;
        let viewport_width = viewport_height * self.image_width as f64 / self.image_height as f64;

        self.w = (self.look_from - self.look_at).unit_vector();
        self.u = self.vup.cross(self.w).unit_vector();
        self.v = self.w.cross(self.u);

        let viewport_u = self.u * viewport_width;
        let viewport_v = self.v * -viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let upper_left_corner =
            self.center - (self.w * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);

        self.pixel00_loc = upper_left_corner + self.pixel_delta_u / 2.0 + self.pixel_delta_v / 2.0;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_get_ray() {
//         let mut outs = Vec::new();
//         let mut camera = Camera::new(16.0 / 9.0, 16, 10, 10);
//         let world = HittableList {
//             objects: vec![
//                 HittableEnum::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)),
//                 HittableEnum::Sphere(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
//             ],
//         };
//         camera.initialize();
//         for i in 0..camera.image_height {
//             for j in 0..camera.image_width {
//                 let pixel_color = (0..camera.samples_per_pixel)
//                     .map(|_| {
//                         let r = camera.get_ray(i, j);
//                         camera.ray_color(r, &world, camera.max_depth)
//                     })
//                     .fold(Vec3(0.0, 0.0, 0.0), |acc, x| acc + x)
//                     * camera.pixel_sample_scale;
//                 outs.push(pixel_color);
//             }
//         }
//         assert!(outs.iter().all(|x| -0.5 <= x.x() && x.x() <= 1.5));
//     }
// }
