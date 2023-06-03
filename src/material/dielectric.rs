use cgmath::{vec3, InnerSpace};

use crate::{utils, Ray, Record, math::Vec3};

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f32,
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / ( 1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Dielectric {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &Record,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > fastrand::f32() {
            direction = utils::reflect(unit_direction, rec.normal);
        } else {
            direction = utils::refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray {
            origin: rec.point,
            direction,
        };

        true
    }
}
