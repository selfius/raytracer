use core::f32;

use crate::buffer::{Buffer, Dimensions, Point, Rgb};
use crate::common::DEBUG_PINK;

use std::{fs::File, io::Error};

pub struct Material {
    pub diffuse_color: Box<dyn ValueGenerator<Rgb> + Send + Sync>,
    pub shininess: f32,
    pub albedo: (f32, f32, f32, f32), // diffuse, specular, reflection
    pub refractive_index: f32,
}

pub trait ValueGenerator<U> {
    fn get(&self, texture_coords: Option<(f32, f32)>) -> U;
}

struct SolidColor(Rgb);
impl ValueGenerator<Rgb> for SolidColor {
    fn get(&self, _: Option<(f32, f32)>) -> Rgb {
        self.0.clone()
    }
}

struct CheckerBoard(Rgb, Rgb);

const CHECKER_BOARD_ROWS: u8 = 8;

impl ValueGenerator<Rgb> for CheckerBoard {
    fn get(&self, texture_coords: Option<(f32, f32)>) -> Rgb {
        let light_color = &self.0;
        let dark_color = &self.1;
        if let Some((x, y)) = texture_coords {
            let x = f32::min(1.0 - f32::EPSILON, x);
            let y = f32::min(1.0 - f32::EPSILON, y);

            let x = (x * (CHECKER_BOARD_ROWS as f32)) as u8;
            let y = (y * (CHECKER_BOARD_ROWS as f32)) as u8;

            return if (x + y) % 2 == 0 {
                light_color.clone()
            } else {
                dark_color.clone()
            };
        }
        DEBUG_PINK.clone()
    }
}

struct ImageTexture {
    image_buffer: Buffer,
}

impl ImageTexture {
    fn load(file_path: &str) -> Result<ImageTexture, Error> {
        let texture_file = File::open(file_path)?;
        let decoder = png::Decoder::new(texture_file);
        let mut reader = decoder.read_info()?;
        let width = reader.info().width;
        let height = reader.info().height;

        let mut buffer = Buffer::new(Dimensions(width, height), 3);

        let mut buf = vec![0; reader.output_buffer_size()];
        reader.next_frame(&mut buf)?;
        let mut buf = buf.into_iter();
        let mut idx = 0;
        let mut idx_without_alpha = 0;
        while let Some(value) = buf.next() {
            if (idx + 1) % 4 == 0 {
                idx += 1;
                continue;
            }
            buffer.set_raw_value(idx_without_alpha, value);
            idx_without_alpha += 1;
            idx += 1;
        }

        Ok(ImageTexture {
            image_buffer: buffer,
        })
    }
}

impl ValueGenerator<Rgb> for ImageTexture {
    fn get(&self, texture_coords: Option<(f32, f32)>) -> Rgb {
        if let Some((x, y)) = texture_coords {
            let x = (f32::round((*self.image_buffer.width() - 1) as f32 * x) as u32)
                .min(*self.image_buffer.width() - 1);
            let y = (f32::round((*self.image_buffer.height() - 1) as f32 * y) as u32)
                .min(*self.image_buffer.height() - 1);

            return self.image_buffer.get(&Point(x, y));
        }
        DEBUG_PINK
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

    pub fn checker_board() -> Material {
        Material {
            diffuse_color: Box::new(CheckerBoard(Rgb::new(179, 118, 62), Rgb::new(67, 45, 35))),
            shininess: 50.0,
            albedo: (0.8, 0.6, 0.1, 0.0),
            refractive_index: 0.0,
        }
    }

    pub fn rubbery_red_checker_board() -> Material {
        Material {
            diffuse_color: Box::new(CheckerBoard(Rgb::new(190, 30, 30), Rgb::new(230, 30, 30))),
            shininess: 10.0,
            albedo: (0.9, 0.1, 0.0, 0.0),
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

    pub fn skysphere() -> Material {
        Material {
            diffuse_color: Box::new(ImageTexture::load("sky.png").unwrap()),
            shininess: 0.0,
            albedo: (1.0, 0.0, 0.0, 0.0),
            refractive_index: 0.0,
        }
    }
}
