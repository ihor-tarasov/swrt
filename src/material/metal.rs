use crate::{random, utils, Ray};
use glam::Vec3;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn scatter(
        &self,
        r_in: &crate::Ray,
        rec: &crate::Record,
        attenuation: &mut Vec3,
        scattered: &mut crate::Ray,
    ) -> bool {
        let reflected = utils::reflect(r_in.direction.normalize(), rec.normal);
        *scattered = Ray {
            origin: rec.point,
            direction: reflected + self.fuzz * random::in_unit_sphere(),
        };
        *attenuation = self.albedo;
        scattered.direction.dot(rec.normal) > 0.0
    }
}
