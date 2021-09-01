use std::ops::*;
use std::io::Write;
use std::fs::File;
// use rand::{Rand, Rng, random};


#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }

    pub fn new(x: f64, y: f64, z: f64) -> Self { Self (x, y, z) }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
    
    pub fn dot(&self, other: Self) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(self.1 * other.2 - self.2 * other.1, self.2 * other.0 - self.0 * other.2, self.0 * other.1 - self.1 * other.0)
    }

    pub fn write_color(&self, mut file: &File) {
        file.write_all(format!("{} {} {}\n", (255.0 * self.x()) as u64, (255.0 * self.y()) as u64, (255.0 * self.z()) as u64).as_bytes()).expect("write failed");
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Self::new(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self {
        Self::new(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        Self::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self (self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self (self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self (self.0 * other, self.1 * other, self.2 * other);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self (self.0 / other, self.1 / other, self.2 / other);
    }
}
