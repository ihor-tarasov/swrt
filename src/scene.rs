use rt::{
    material,
    object::{self, Hit},
    random, vec3, Camera, Renderer,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub samples_per_step: usize,
    pub ray_depth: usize,
    pub camera: Camera,
    pub world: Hit,
}

fn random_scene() -> Hit {
    let mut world = object::hit_list();

    let ground_material = material::lambertian(vec3(0.5, 0.5, 0.5));
    world.push(object::sphere(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in (-11)..11 {
        for b in (-11)..11 {
            let choose_mat = fastrand::f32();
            let center = vec3(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );

            if (center - vec3(4.0, 0.2, 0.0)).length_squared() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo =
                        rt::utils::mul_per_comp(random::vec(0.0..1.0), random::vec(0.0..1.0));
                    sphere_material = material::lambertian(albedo);
                    world.push(object::sphere(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = random::vec(0.5..1.0);
                    let fuzz = random::f32(0.0..0.5);
                    sphere_material = material::metal(albedo, fuzz);
                    world.push(object::sphere(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = material::dielectric(1.5);
                    world.push(object::sphere(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = material::dielectric(1.5);
    world.push(object::sphere(vec3(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = material::lambertian(vec3(0.4, 0.2, 0.1));
    world.push(object::sphere(vec3(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = material::metal(vec3(0.7, 0.6, 0.5), 0.0);
    world.push(object::sphere(vec3(4.0, 1.0, 0.0), 1.0, material3));

    world
}

impl Default for Scene {
    fn default() -> Self {
        let width = 1920;
        let height = 1080;
        let world = random_scene();

        let lookfrom = vec3(13.0, 2.0, 3.0);
        let lookat = vec3(0.0, 0.0, 0.0);

        let camera = Camera::new(
            lookfrom,
            lookat,
            vec3(0.0, 1.0, 0.0),
            20.0,
            width as f32,
            height as f32,
            0.1,
            10.0,
        );

        Self {
            width,
            height,
            samples_per_pixel: 500,
            samples_per_step: 20,
            ray_depth: 50,
            camera,
            world,
        }
    }
}

impl Into<Renderer> for Scene {
    fn into(self) -> Renderer {
        Renderer {
            width: self.width as f32,
            height: self.height as f32,
            samples_per_pixel: self.samples_per_pixel,
            ray_depth: self.ray_depth,
            camera: self.camera,
            world: self.world,
        }
    }
}
