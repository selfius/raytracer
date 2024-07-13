mod buffer;

use std::fs::File;
use png::Encoder;
use buffer::{Buffer, Dimensions, Point, Rgb};

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
        .write_image_data(buffer.get_data_ref())
        .expect("Something went wrong lol");
}

fn draw(buffer: &mut Buffer) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            buffer.set(
                Point(x, y),
                Rgb((x * 256 / WIDTH) as u8, 0, (y * 256 / HEIGHT) as u8),
            );
        }
    }
}
