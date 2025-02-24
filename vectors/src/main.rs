mod camera;
mod hittable;
mod interval;
mod ray;
mod utils;
mod vec3;
use camera::*;
use hittable::*;
use ray::*;
use std::io::Write;
use vec3::*;

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
    const PIXEL_SAMPLE_SCALE: i32 = 30;

    let world: HittableList = HittableList {
        objects: vec![
            HittableEnum::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5)),
            HittableEnum::Sphere(Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0)),
        ],
    };
    let output = {
        let mut file = std::fs::File::create("./images/test.ppm").expect("create failed");
        move |text: String| {
            file.write_all(text.as_bytes()).expect("write failed");
        }
    };
    Camera::new(ASPECT_RATIO, IMAGE_WIDTH, PIXEL_SAMPLE_SCALE).render(output, world);

    //Render
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
