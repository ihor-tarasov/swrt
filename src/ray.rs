use crate::{object::Hit, utils, Record, Vec3, vec3};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: vec3(0.0, 0.0, 0.0),
            direction: vec3(0.0, 0.0, 0.0),
        }
    }
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn trace(&self, world: &Hit, depth: usize) -> Vec3 {
        let mut rec = Record::default();

        if depth == 0 {
            return vec3(0.0, 0.0, 0.0);
        }

        if world.hit(self, 0.001, f32::INFINITY, &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = vec3(0.0, 0.0, 0.0);
            if rec
                .mat
                .scatter(self, &rec, &mut attenuation, &mut scattered)
            {
                utils::mul_per_comp(attenuation, scattered.trace(world, depth - 1))
            } else {
                vec3(0.0, 0.0, 0.0)
            }
        } else {
            let norm = self.direction.normalize();
            let t = 0.5 * (norm.y() + 1.0);
            (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
        }
    }
}
