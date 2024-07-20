use std::ops::Mul;
pub struct Point(pub u32, pub u32);

pub struct Dimensions(pub u32, pub u32);

pub struct Buffer {
    data: Vec<u8>,
    width: u32,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl Mul<f32> for Rgb {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let new_r = self.0 as f32 * rhs;
        let new_g = self.1 as f32 * rhs;
        let new_b = self.2 as f32 * rhs;
        assert!(
            new_r <= 256.0,
            "expected value in 0..256 range got {}",
            new_r
        );
        assert!(
            new_g <= 256.0,
            "expected value in 0..256 range got {}",
            new_g
        );
        assert!(
            new_b <= 256.0,
            "expected value in 0..256 range got {}",
            new_b
        );
        Rgb((new_r) as u8, (new_g) as u8, (new_b) as u8)
    }
}

#[cfg(test)]
mod rgb_test {
    use super::*;

    #[test]
    fn rgb_multiplication() {
        assert_eq!(Rgb(100, 10, 200) * 0.8, Rgb(80, 8, 160));
    }

    #[test]
    #[should_panic]
    fn rgb_multiplication_out_of_bound() {
        assert_eq!(Rgb(100, 10, 200) * 2.0, Rgb(80, 8, 160));
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
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    pub fn set(&mut self, point: &Point, rgb: &Rgb) {
        let first_byte = ((point.1 * self.width + point.0) * 3) as usize;
        self.data[first_byte] = rgb.0;
        self.data[first_byte + 1] = rgb.1;
        self.data[first_byte + 2] = rgb.2;
    }

    pub fn get_data_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}
