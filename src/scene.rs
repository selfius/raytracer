mod mesh;
mod rect;
pub mod sphere;
mod triangle;

use crate::buffer::Rgb;
use crate::vector_math::Vec3;

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
                material: &RUBBERY_RED,
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(2.5, 0.8, -7.0),
                    radius: 1.0,
                }),
                material: &GLOSSY_GREEN,
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(-2.5, 0.2, -5.0),
                    radius: 1.0,
                }),
                material: &GLOSSY_BLUE,
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(4.5, 5.2, -11.0),
                    radius: 2.5,
                }),
                material: &MIRROR,
            },
            Object {
                surface: Box::new(Sphere {
                    origin: Vec3::new(-1.2, -0.6, -4.0),
                    radius: 0.7,
                }),
                material: &GLASS,
            },
            Object {
                surface: Box::new(Mesh::from_obj_file("cube.obj")),
                material: &RUBBERY_RED,
            },
            Object {
                surface: Box::new(Rect::new(
                    Vec3::new(-1.0, -2.0, -5.0),
                    Vec3::new(-1.0, -2.0, -9.0),
                    Vec3::new(3.0, -2.0, -9.0),
                    Vec3::new(3.0, -2.0, -5.0),
                )),
                material: &GLOSSY_GREEN,
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
    }
}

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

pub trait Surface {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection>;
}

pub struct Object {
    pub material: &'static Material,
    pub surface: Box<dyn Surface + Send + Sync>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub diffuse_color: Rgb,
    pub shininess: f32,
    pub albedo: (f32, f32, f32, f32), // diffuse, specular, reflection
    pub refractive_index: f32,
}

pub struct Light {
    pub origin: Vec3,
    pub intensity: f32,
}

static GLOSSY_BLUE: Material = Material {
    diffuse_color: Rgb::new(50, 50, 170),
    shininess: 200.0,
    albedo: (0.5, 0.5, 0.0, 0.0),
    refractive_index: 0.0,
};

static RUBBERY_RED: Material = Material {
    diffuse_color: Rgb::new(190, 30, 30),
    shininess: 10.0,
    albedo: (0.9, 0.1, 0.0, 0.0),
    refractive_index: 0.0,
};

static GLOSSY_GREEN: Material = Material {
    diffuse_color: Rgb::new(50, 250, 50),
    shininess: 50.0,
    albedo: (0.8, 0.6, 0.1, 0.0),
    refractive_index: 0.0,
};

static MIRROR: Material = Material {
    diffuse_color: Rgb::new(10, 10, 10),
    shininess: 200.0,
    albedo: (0.2, 0.6, 0.8, 0.0),
    refractive_index: 0.0,
};

static GLASS: Material = Material {
    diffuse_color: Rgb::new(10, 10, 10),
    shininess: 200.0,
    albedo: (0.0, 0.6, 0.0, 0.9),
    refractive_index: 1.8,
};
