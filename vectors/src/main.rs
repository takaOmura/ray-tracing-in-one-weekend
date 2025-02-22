mod hittable;
mod ray;
mod sphere;
mod vec3;
use crate::hittable::*;
use crate::sphere::Sphere;
use ray::*;
use std::io::Write;
use vec3::*;

pub fn ray_color(r: Ray, world: &dyn Hittable, _f: &dyn Fn() -> bool) -> Color {
    let mut rec = HitRecord {
        point: Vec3(0.0, 0.0, 0.0),
        normal: Vec3(0.0, 0.0, 0.0),
        t: 0.0,
        front_face: false,
    };

    if world.hit(&r, 0.0, f64::INFINITY, &mut rec) {
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

    // let t = hit_sphere(&Vec3(0.0, 0.0, -1.0), 0.5, &r);
    // if  t > 0.0 {
    //     let n = r.at(t) - Vec3( 0.0,0.0,-1.0);
    //     return Vec3(n.x()+1.0, n.y()+1.0, n.z()+1.0) * 0.5;
    // }
    // let unit_direction: Vec3 = r.dir.unit_vector();
    // let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    // Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

pub fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = *center - r.origin;
    let a = r.dir.length_squared();
    let h = r.dir.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const fn calc_height() -> i32 {
        let image_height = IMAGE_WIDTH / ASPECT_RATIO as i32;
        if image_height < 1 {
            1
        } else {
            image_height
        }
    }
    const IMAGE_HEIGHT: i32 = calc_height();

    // world
    //
    let sphere: Box<dyn Hittable> = Box::new(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5));
    let big_sphere: Box<dyn Hittable> = Box::new(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0));
    let world: HittableList = HittableList {
        objects: vec![&sphere, &big_sphere],
    };

    // camera
    const CAMERA_CENTER: Vec3 = Vec3(0.0, 0.0, 0.0);

    //viewport
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64;
    const FOCAL_LENGTH: f64 = 1.0;

    // Viewport
    // u:vector from topleft to right
    // v:vector from topleft to bottom
    const VIEWPORT_U: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3(0.0, -1.0 * VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u: Vec3 = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v: Vec3 = VIEWPORT_V / IMAGE_HEIGHT as f64;
    let upper_left_corner: Vec3 =
        CAMERA_CENTER - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH);
    // pixel of upper left corner
    let pixel00_loc: Vec3 = upper_left_corner + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    //Render
    let mut file = std::fs::File::create("test.ppm").expect("create failed");
    file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes())
        .expect("write failed");
    for j in 0..(IMAGE_HEIGHT) as i32 {
        // if j % 10 == 0 {
        //     println!("line remaining: {}", j);
        // }
        for i in 0..IMAGE_WIDTH {
            let pixel_center: Vec3 =
                pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - CAMERA_CENTER;
            let ray = Ray {
                origin: CAMERA_CENTER,
                dir: ray_direction,
            };
            // if j % 100 == 0 && i == 0 {
            //     println!("ray_direction {:?}", ray_direction);
            // }
            let pixel_color: Color = ray_color(ray, &world, &|| {
                (j == 0 && i == 0) || j % 100 == 0 && i == 0
            });
            Vec3::write_color(&pixel_color, &file);
        }
    }
    // print each element of upper_left_corner
    // println!("upper left corner --- {:?}", upper_left_corner);
    // println!("{:?}", pixel00_loc);
    // println!("{:?}", pixel_delta_u);
    // println!("{:?}", pixel_delta_v);
    //// let vecneg = vec1 - vec2;
    // let vecmul = vec1 * 2.0;
    // let vecdiv = vec1 / 2.0;
    // println!("{:?}", vecadd);
    // println!("{:?}", vecneg);
    // println!("{:?}", vecmul);
    // println!("{:?}", vecdiv);
    // println!("{:?}", Vec3(1.0, 0.0, 0.0).length());
    // println!("{:?}", Vec3(1.0, 0.0, 0.0).unit_vector());
}
