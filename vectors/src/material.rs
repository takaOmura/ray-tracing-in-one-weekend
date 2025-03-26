use crate::utils::*;
use crate::{ray::Ray, vec3::*};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Vec3, f64),
    Dielectric(f64),
    None,
}

impl Material {
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn new_metal(albedo: Vec3, fuzz: f64) -> Material {
        let f = {
            if fuzz < 1.0 {
                fuzz
            } else {
                1.0
            }
        };
        Material::Metal(albedo, f)
    }

    pub fn scatter(
        &self,
        r_in: &Ray,
        point: Point3,
        normal: Vec3,
        front_face: bool,
    ) -> (bool, Ray, Color) {
        match self {
            // albedo is the whiteness of the material
            Material::Lambertian(albedo) => {
                let scatter_direction = {
                    match near_zero(normal + random_unit_vector()) {
                        true => normal,
                        false => normal + random_unit_vector(),
                    }
                };
                (
                    true,
                    Ray {
                        origin: point,
                        dir: scatter_direction,
                    },
                    *albedo,
                )
            }

            Material::Metal(albedo, fuzz) => {
                let reflected = reflect(r_in.dir.unit_vector(), normal);
                (
                    true,
                    Ray {
                        origin: point,
                        dir: reflected + random_unit_vector() * *fuzz,
                    },
                    *albedo,
                )
                // scattered.dir.dot(normal) > 0.0
            }

            Material::Dielectric(refraction_index) => {
                let attenuation = Vec3(1.0, 1.0, 1.0);

                let ri = if front_face {
                    1.0 / *refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = r_in.dir.unit_vector();
                let cos_theta = (-unit_direction).dot(normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let cannot_refract = ri * sin_theta > 1.0;

                if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
                    (
                        true,
                        Ray {
                            origin: point,
                            dir: reflect(unit_direction, normal),
                        },
                        attenuation,
                    )
                } else {
                    (
                        true,
                        Ray {
                            origin: point,
                            dir: refract(unit_direction, normal, ri),
                        },
                        attenuation,
                    )
                }
            }

            _ => (
                false,
                Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)),
                Vec3(0.0, 0.0, 0.0),
            ),
        }
    }
}
