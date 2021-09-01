mod vec3;
mod ray;
use vec3::*;
use ray::*;
use std::io::Write;

pub fn ray_color(r: Ray) -> Vec3 {
    let unit_direction: Vec3 = r.dir.unit_vector();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t 
}

fn main () {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 200;
    const IMAGE_HEIGHT: i32 = 100; // (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    //camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Vec3 = Vec3(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = ORIGIN - horizontal / 2.0  - vertical / 2.0 - Vec3(0.0, 0.0, FOCAL_LENGTH);

    //Render
    let mut file = std::fs::File::create("test.ppm").expect("create failed");
    file.write_all(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT).as_bytes()).expect("write failed");
    for i in (0..(IMAGE_HEIGHT)as i32).rev() {
        println!("line remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let u = j as f64 / ((IMAGE_WIDTH - 1) as f64);
            let v = i as f64 / ((IMAGE_HEIGHT - 1) as f64);
            
            let r: Ray = Ray{
                origin: ORIGIN,
                dir: lower_left_corner + horizontal * u + vertical * v - ORIGIN
            };
            let pixel_color: Vec3 = ray_color(r);
            Vec3::write_color(&pixel_color, &file);
        }
    }
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

