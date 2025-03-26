mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod utils;
mod vec3;
use camera::*;
use hittable::*;
use ray::*;
use std::io::Write;
use vec3::*;

fn main() {
    //image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const PIXEL_SAMPLE_SCALE: i32 = 150;
    const MAX_DEPTH: i32 = 40;
    const VFOV: f64 = 20.0;
    const LOOK_FROM: Vec3 = Vec3(-2.0, 1.0, 3.0);
    const LOOK_AT: Vec3 = Vec3(0.0, 0.0, -1.0);
    const VUP: Vec3 = Vec3(0.0, 1.0, 0.0);

    let material_ground = material::Material::Lambertian(Vec3(0.8, 0.8, 0.0));
    let material_center = material::Material::Lambertian(Vec3(0.1, 0.2, 0.5));
    // let material_surrounding = material::Material::new_metal(Vec3(1.0, 1.0, 1.0), 0.0);
    let material_left = material::Material::Dielectric(1.5);
    let material_bubble = material::Material::Dielectric(1.0 / 1.614);
    let material_inside_bubble1 = material::Material::Lambertian(Vec3(0.2, 0.8, 0.6));
    let material_inside_bubble2 = material::Material::Lambertian(Vec3(0.8, 1.0, 0.2));
    let material_inside_bubble3 = material::Material::new_metal(Vec3(0.8, 1.0, 0.8), 0.0);
    let material_right = material::Material::new_metal(Vec3(0.8, 0.8, 0.8), 0.8);

    let world: HittableList = HittableList {
        objects: vec![
            HittableEnum::HalfSphere(HalfSphere::new(
                Vec3(0.0, -100.5, -1.0),
                100.0,
                material_ground,
                Vec3(0.0, 1.0, 0.0),
            )),
            HittableEnum::Sphere(Sphere::new(Vec3(0.0, 0.0, -1.2), 0.5, material_center)),
            HittableEnum::Sphere(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.5, material_left)),
            HittableEnum::Sphere(Sphere::new(Vec3(-1.0, 0.0, -1.0), 0.48, material_bubble)),
            HittableEnum::Sphere(Sphere::new(
                Vec3(-0.9, -0.1, -0.9),
                0.2,
                material_inside_bubble1,
            )),
            HittableEnum::Sphere(Sphere::new(
                Vec3(-1.1, 0.1, -1.1),
                0.2,
                material_inside_bubble2,
            )),
            HittableEnum::Sphere(Sphere::new(
                Vec3(-0.8, 0.2, -1.0),
                0.2,
                material_inside_bubble3,
            )),
            HittableEnum::Sphere(Sphere::new(Vec3(1.0, 0.0, -1.0), 0.5, material_right)),
        ],
    };

    let output = {
        let mut file = std::fs::File::create("./images/playaround.ppm").expect("create failed");
        move |text: String| {
            file.write_all(text.as_bytes()).expect("write failed");
        }
    };

    Camera::new(
        ASPECT_RATIO,
        VFOV,
        IMAGE_WIDTH,
        PIXEL_SAMPLE_SCALE,
        MAX_DEPTH,
        LOOK_FROM,
        LOOK_AT,
        VUP,
    )
    .render(output, world);

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
