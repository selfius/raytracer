use png::Encoder;
use std::fs::File;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const CHANNELS: u8 = 3;


fn main() {
    let output = File::create("test.png").expect("error when creating the output file");
    let mut encoder = Encoder::new(output, WIDTH, HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut buffer = Buffer::new(Dimensions(WIDTH, HEIGHT), CHANNELS);
    buffer.clear();
    draw(&mut buffer);

    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(&buffer.data)
        .expect("Something went wrong lol");
}

fn draw(buffer: &mut Buffer) {
    for x in 0..buffer.width {
        for y in 0..buffer.height {
            buffer.set(
                Point(x, y),
                Rgb(
                    (x * 256 / buffer.width) as u8,
                    0,
                    (y * 256 / buffer.height) as u8,
                ),
            );
        }
    }
}

struct Point(u32, u32);

struct Dimensions(u32, u32);

struct Buffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

struct Rgb(u8, u8, u8);

impl Buffer {
    fn new(dimensions: Dimensions, depth: u8) -> Buffer {
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

    fn clear(&mut self) {
        for i in 0..self.data.len() {
            self.data[i] = 0;
        }
    }

    fn set(&mut self, point: Point, rgb: Rgb) {
        let first_byte = ((point.1 * self.width + point.0) * 3) as usize;
        self.data[first_byte] = rgb.0;
        self.data[first_byte + 1] = rgb.1;
        self.data[first_byte + 2] = rgb.2;
    }
}
