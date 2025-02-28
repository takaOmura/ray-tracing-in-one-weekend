use crate::{ray::Ray, vec3::*};

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Vec3),
    // Dielectric(f64),
    None,
}

impl Material {
    /// .
    pub fn scatter(&self, r_in: &Ray, point: Point3, normal: Vec3) -> (bool, Ray, Color) {
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
            Material::Metal(albedo) => {
                let reflected = reflect(r_in.dir.unit_vector(), normal);
                (
                    true,
                    Ray {
                        origin: point,
                        dir: reflected,
                    },
                    *albedo,
                )
                // scattered.dir.dot(normal) > 0.0
            }

            _ => (
                false,
                Ray::new(Vec3(0.0, 0.0, 0.0), Vec3(0.0, 0.0, 0.0)),
                Vec3(0.0, 0.0, 0.0),
            ),
        }
    }
}
