use raytracer::buffer::{Buffer, Dimensions, Point, Rgb};
use png::Encoder;
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
    background(&mut buffer);
    raytracer::draw(&mut buffer);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buffer.get_data_ref())?;
    Ok(())
}

fn background(buffer: &mut Buffer) {
    for x in 0..raytracer::WIDTH {
        for y in 0..raytracer::HEIGHT {
            buffer.set(
                &Point(x, y),
                &Rgb::new(
                    (x * 256 / raytracer::WIDTH) as u8,
                    0,
                    (y * 256 / raytracer::HEIGHT) as u8,
                ),
            );
        }
    }
}
