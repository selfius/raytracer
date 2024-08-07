use std::{
    ops::{Add, Mul},
};
pub struct Point(pub u32, pub u32);

pub struct Dimensions(pub u32, pub u32);

pub struct Buffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Rgb {
    values: [u8; 3],
}

impl Rgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Rgb {
        Rgb { values: [r, g, b] }
    }
}

impl Mul<f32> for Rgb {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self::Output {
        for i in &mut self.values {
            *i = (*i as f32 * rhs) as u8;
        }
        self
    }
}

impl Add for Rgb {
    type Output = Self;
    fn add(self, v2: Self) -> Self {
        let mut new_values = [0_u8; 3];
        for (idx, (v1_i, v2_i)) in self.values.iter().zip(v2.values).enumerate() {
            let value_with_overflow = *v1_i as u16 + v2_i as u16;
            new_values[idx] = value_with_overflow.min(255) as u8;
        }
        Self { values: new_values }
    }
}

#[cfg(test)]
mod rgb_test {
    use super::*;

    #[test]
    fn rgb_multiplication() {
        assert_eq!(Rgb::new(100, 10, 200) * 0.8, Rgb::new(80, 8, 160));
    }

    #[test]
    fn rgb_multiplication_out_of_bound() {
        assert_eq!(Rgb::new(100, 10, 200) * 2.0, Rgb::new(200, 20, 255));
    }

    #[test]
    fn rgb_addition() {
        assert_eq!(
            Rgb::new(100, 10, 200) + Rgb::new(100, 10, 200),
            Rgb::new(200, 20, 255)
        );
    }
}

impl Buffer {
    pub fn new(dimensions: Dimensions, depth: u8) -> Buffer {
        let mut data: Vec<u8> =
            Vec::with_capacity((dimensions.0 * dimensions.1 * depth as u32) as usize);
        for _i in 0..data.capacity() {
            data.push(0);
        }
        Buffer {
            data,
            width: dimensions.0,
            height: dimensions.1,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    pub fn set(&mut self, point: &Point, rgb: &Rgb) {
        let first_byte = ((point.1 * self.width + point.0) * 3) as usize;
        self.data[first_byte] = rgb.values[0];
        self.data[first_byte + 1] = rgb.values[1];
        self.data[first_byte + 2] = rgb.values[2];
    }

    pub fn get(&self, point: &Point) -> Rgb {
        let first_byte = ((point.1 * self.width + point.0) * 3) as usize;
        Rgb::new(
            self.data[first_byte],
            self.data[first_byte + 1],
            self.data[first_byte + 2],
        )
    }

    pub fn set_raw_value(&mut self, idx: usize, value: u8) {
        self.data[idx] = value;
    }

    pub fn get_data_ref(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn width(&self) -> &u32 {
        &self.width
    }

    pub fn height(&self) -> &u32 {
        &self.height
    }
}
