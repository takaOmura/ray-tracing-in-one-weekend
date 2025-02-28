use crate::interval::*;
use crate::utils::*;
use std::ops::*;
// use rand::{Rand, Rng, random};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub fn linear_to_gamma(value: f64) -> f64 {
    match value {
        value if value >= 0.0 => value.sqrt(),
        _ => 0.0,
    }
}

pub fn near_zero(v: Vec3) -> bool {
    const S: f64 = 1e-8;
    v.0.abs() < S && v.1.abs() < S && v.2.abs() < S
}

fn random() -> Vec3 {
    Vec3(random_double(), random_double(), random_double())
}

fn random_with_range(min: f64, max: f64) -> Vec3 {
    Vec3(
        random_double_with_range(min, max),
        random_double_with_range(min, max),
        random_double_with_range(min, max),
    )
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = random_with_range(-1.0, 1.0);
        let length_squared = p.length_squared();
        if 1e-160 < length_squared && length_squared < 1.0 {
            return p.unit_vector();
        }
    }
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

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
        Self::new(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    const INTENSITY: Interval = Interval { min: 0.0, max: 1.0 };

    pub fn get_color(&self) -> String {
        let r = linear_to_gamma(self.x());
        let g = linear_to_gamma(self.y());
        let b = linear_to_gamma(self.z());
        let r_byte = (255.999 * Self::INTENSITY.clamp(r)) as u64;
        let g_byte = (255.999 * Self::INTENSITY.clamp(g)) as u64;
        let b_byte = (255.999 * Self::INTENSITY.clamp(b)) as u64;
        format!("{} {} {}\n", r_byte, g_byte, b_byte)
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

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.0, -self.1, -self.2)
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
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self(self.0 * other, self.1 * other, self.2 * other);
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self = Self(self.0 / other, self.1 / other, self.2 / other);
    }
}
