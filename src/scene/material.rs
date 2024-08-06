use crate::buffer::Rgb;

pub struct Material {
    pub diffuse_color: Box<dyn ValueGenerator<Rgb> + Send + Sync>,
    pub shininess: f32,
    pub albedo: (f32, f32, f32, f32), // diffuse, specular, reflection
    pub refractive_index: f32,
}

pub trait ValueGenerator<U> {
    fn get(&self, texture_coords: Option<(f32, f32)>) -> &U;
}

struct SolidColor(Rgb);
impl ValueGenerator<Rgb> for SolidColor {
    fn get(&self, _: Option<(f32, f32)>) -> &Rgb {
        &self.0
    }
}

pub struct Materials {}

impl Materials {
    pub fn solid_color(color: Rgb) -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(color)),
            shininess: 0.0,
            albedo: (0.5, 0.5, 0.0, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn glossy_blue() -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(Rgb::new(50, 50, 170))),
            shininess: 200.0,
            albedo: (0.5, 0.5, 0.0, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn rubbery_red() -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(Rgb::new(190, 30, 30))),
            shininess: 10.0,
            albedo: (0.9, 0.1, 0.0, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn glossy_green() -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(Rgb::new(50, 250, 50))),
            shininess: 50.0,
            albedo: (0.8, 0.6, 0.1, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn mirror() -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(Rgb::new(10, 10, 10))),
            shininess: 200.0,
            albedo: (0.2, 0.6, 0.8, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn glass() -> Material {
        Material {
            diffuse_color: Box::new(SolidColor(Rgb::new(10, 10, 10))),
            shininess: 200.0,
            albedo: (0.0, 0.6, 0.0, 0.9),
            refractive_index: 1.8,
        }
    }
}
