use std::io::Write;

fn main() {
    let nx = 256;
    let ny = 256;

    let mut file = std::fs::File::create("test.ppm").expect("create failed");
    file.write_all(format!("P3\n{} {}\n255\n", nx, ny).as_bytes()).expect("write failed");
    for i in (0..ny).rev() {
        for j in 0..nx {
            
            let r: f64 = (i as f64) / ((nx - 1) as f64);
            let g: f64 = (j as f64) / ((ny - 1) as f64);
            let b: f64 = 0.25;

            let ir = (255.99 * r) as u16;
            let ig = (255.99 * g) as u16;
            let ib = (255.99 * b) as u16;

            file.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes()).expect("write failed");

        }
    }
}
