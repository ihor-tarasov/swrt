use crate::{Ray, Record, math::Vec3};
use self::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

mod dielectric;
mod lambertian;
mod metal;

#[derive(Clone)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &Record,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            Material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            Material::Dielectric(d) => d.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

pub fn lambertian(albedo: Vec3) -> Material {
    Material::Lambertian(Lambertian { albedo })
}

pub fn metal(albedo: Vec3, fuzz: f32) -> Material {
    Material::Metal(Metal { albedo, fuzz })
}

pub fn dielectric(ir: f32) -> Material {
    Material::Dielectric(Dielectric { ir })
}
