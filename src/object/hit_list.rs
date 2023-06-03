use serde::{Serialize, Deserialize};

use crate::{Ray, Record};

use super::Hit;

#[derive(Serialize, Deserialize)]
pub struct HitList(Vec<Hit>);

impl HitList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, h: Hit) {
        self.0.push(h);
    }
}

impl HitList {
    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut Record) -> bool {
        let mut temp_rec = Record::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        self.0.iter().for_each(|h| {
            if h.hit(r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        });

        hit_anything
    }
}
