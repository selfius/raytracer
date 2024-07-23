use png::Encoder;
use raytracer::buffer::{Buffer, Dimensions};
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let output = File::create("test.png")?;
    let mut encoder = Encoder::new(output, raytracer::WIDTH, raytracer::HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut buffer = Buffer::new(
        Dimensions(raytracer::WIDTH, raytracer::HEIGHT),
        raytracer::CHANNELS,
    );
    buffer.clear();
    raytracer::draw(&mut buffer);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buffer.get_data_ref())?;
    Ok(())
}
