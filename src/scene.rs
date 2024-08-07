pub mod material;
mod mesh;
mod rect;
pub mod sphere;
mod triangle;

use crate::vector_math::Vec3;
use material::{Material, Materials};

use crate::ray_tracing::Intersection;

use mesh::Mesh;
use rect::Rect;
use sphere::Sphere;

pub fn create_scene() -> Scene {
    Scene {
        objects: vec![
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(1.7, 1.8, -10.0),
                    radius: 2.0,
                }),
                material: Materials::rubbery_red_checker_board(),
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(2.5, 0.8, -7.0),
                    radius: 1.0,
                }),
                material: Materials::glossy_green(),
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(-2.5, 0.2, -5.0),
                    radius: 1.0,
                }),
                material: Materials::glossy_blue(),
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(4.5, 5.2, -11.0),
                    radius: 2.5,
                }),
                material: Materials::mirror(),
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(-1.2, -0.6, -4.0),
                    radius: 0.7,
                }),
                material: Materials::glass(),
            },
            Object {
                surface: Box::new(Mesh::from_obj_file("cube.obj")),
                material: Materials::rubbery_red(),
            },
            Object {
                surface: Box::new(Rect::new(
                    Vec3::new(-1.0, -2.0, -5.0),
                    Vec3::new(-1.0, -2.0, -9.0),
                    Vec3::new(3.0, -2.0, -9.0),
                    Vec3::new(3.0, -2.0, -5.0),
                )),
                material: Materials::checker_board(),
            },
        ],
        lights: vec![
            Light {
                origin: Vec3::new(10.0, 14.0, 10.0),
                intensity: 0.4,
            },
            Light {
                origin: Vec3::new(5.0, 0.5, -4.0),
                intensity: 0.8,
            },
        ],
        sky_sphere: Materials::skysphere(),
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
    pub sky_sphere: Material,
}

pub trait Surface {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection>;
}

pub struct Object {
    pub material: Material,
    pub surface: Box<dyn Surface + Send + Sync>,
}

pub struct Light {
    pub origin: Vec3,
    pub intensity: f32,
}
