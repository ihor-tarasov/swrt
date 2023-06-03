use serde::{Serialize, Deserialize};

use crate::{Ray, Record, material::Material, Vec3};

use self::{hit_list::HitList, sphere::Sphere};

mod hit_list;
mod sphere;

#[derive(Serialize, Deserialize)]
pub enum Hit {
    Sphere(Box<Sphere>),
    HitList(Box<HitList>),
}

impl Hit {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Record) -> bool {
        match self {
            Hit::Sphere(s) => s.hit(r, t_min, t_max, rec),
            Hit::HitList(h) => h.hit(r, t_min, t_max, rec),
        }
    }

    pub fn push(&mut self, h: Hit) {
        if let Hit::HitList(list) = self {
            list.push(h);
        } else {
            let mut list = HitList::new();
            list.push(h);
            let self_to_push = std::mem::replace(self, Hit::HitList(Box::new(list)));
            match self {
                Hit::HitList(list) => list.push(self_to_push),
                _ => panic!(),
            }
        }
    }
}

pub fn sphere(center: Vec3, radius: f32, mat: Material) -> Hit {
    Hit::Sphere(Box::new(Sphere {
        center,
        radius,
        mat,
    }))
}

pub fn hit_list() -> Hit {
    Hit::HitList(Box::new(HitList::new()))
}
