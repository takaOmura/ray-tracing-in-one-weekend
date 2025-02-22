
// const double infinity = std::numeric_limits<double>::infinity();
// const double pi = 3.1415926535897932385;

const INFINITY = f64::INFINITY;
const PI: f64 = std::f64::consts::PI;

#[inline]
fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
