use crate::{random, utils, Ray, math::Vec3};

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn scatter(
        &self,
        _r_in: &crate::Ray,
        rec: &crate::Record,
        attenuation: &mut Vec3,
        scattered: &mut crate::Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random::unit_vector();

        // Catch degenerate scatter direction
        if utils::near_zero(scatter_direction) {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {
            origin: rec.point,
            direction: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}
