mod buffer;
mod vector_math;

use buffer::{Buffer, Dimensions, Point, Rgb};
use png::Encoder;
use std::fs::File;
use vector_math::Vec3;

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

    //testing ground
    println!(
        "dot product is {}",
        Vec3::new(1.0, 1.0, 2.0).dot_product(&Vec3::new(2.0, 2.0, 3.0))
    );

    println!(
        "vector and scalar product {:?}",
        Vec3::new(1.0, 2.0, -3.0).multiply(3.0)
    );

    println!("magnitude is {}", Vec3::new(2.0, 0.0, 0.0).magnitude());
    println!("normalized {:?}", Vec3::new(2.0, 2.0, 0.0).normalize());
}
