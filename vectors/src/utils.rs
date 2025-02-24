use rand::prelude::*;

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_double_with_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(min..max)
}
