pub struct Point(pub u32, pub u32);

pub struct Dimensions(pub u32, pub u32);

pub struct Buffer {
    data: Vec<u8>,
    width: u32,
}

pub struct Rgb(pub u8, pub u8, pub u8);

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

    pub fn set(&mut self, point: Point, rgb: Rgb) {
        let first_byte = ((point.1 * self.width + point.0) * 3) as usize;
        self.data[first_byte] = rgb.0;
        self.data[first_byte + 1] = rgb.1;
        self.data[first_byte + 2] = rgb.2;
    }

    pub fn get_data_ref(&self) -> &[u8] {
        self.data.as_slice()
    }
}
