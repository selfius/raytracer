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

    let mut data: Vec<u8> = Vec::with_capacity((WIDTH * HEIGHT * CHANNELS as u32) as usize);
    for i in 0..data.capacity() / 3 {
        let x = i as u32 % WIDTH;
        let y = i as u32 / WIDTH;

        data.push((x * 256 / WIDTH) as u8);
        data.push((y * 256 / HEIGHT) as u8);
        data.push(0);
    }
    let mut writer = encoder.write_header().unwrap();
    writer
        .write_image_data(&data)
        .expect("Something went wrong lol");
}
