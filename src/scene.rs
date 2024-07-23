use crate::buffer::Rgb;
use crate::vector_math::Vec3;

pub fn create_scene() -> Scene {
    Scene {
        spheres: vec![
            Sphere {
                origin: Vec3::new(3.2, 1.8, -10.0),
                radius: 2.0,
                material: &RUBBERY_RED,
            },
            Sphere {
                origin: Vec3::new(4.0, 0.8, -7.0),
                radius: 1.0,
                material: &GLOSSY_GREEN,
            },
            Sphere {
                origin: Vec3::new(-1.0, 0.2, -5.0),
                radius: 1.0,
                material: &GLOSSY_BLUE,
            },
            Sphere {
                origin: Vec3::new(-2.2, 4.5, -16.0),
                radius: 4.0,
                material: &GLOSSY_BLUE,
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
    pub spheres: Vec<Sphere>,
    pub lights: Vec<Light>,
}

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
    pub material: &'static Material,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub diffuse_color: Rgb,
    pub shininess: f32,
    pub albedo: (f32, f32), // what portion of light reflects diffusely and what reflects specularely
}
pub struct Light {
    pub origin: Vec3,
    pub intensity: f32,
}

static GLOSSY_BLUE: Material = Material {
    diffuse_color: Rgb::new(50, 50, 170),
    shininess: 200.0,
    albedo: (0.5, 0.5),
};

static RUBBERY_RED: Material = Material {
    diffuse_color: Rgb::new(190, 30, 30),
    shininess: 10.0,
    albedo: (0.9, 0.1),
};

static GLOSSY_GREEN: Material = Material {
    diffuse_color: Rgb::new(50, 250, 50),
    shininess: 50.0,
    albedo: (0.8, 0.6),
};
