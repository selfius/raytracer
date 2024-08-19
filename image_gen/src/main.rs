use lib::buffer::{Buffer, Dimensions};
use lib::vector_math::Vec3;
use png::Encoder;
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let output = File::create("test.png")?;
    let mut encoder = Encoder::new(output, lib::WIDTH, lib::HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut buffer = Buffer::new(Dimensions(lib::WIDTH, lib::HEIGHT), lib::CHANNELS);
    buffer.clear();
    lib::draw(
        &mut buffer,
        Vec3::new(0.0, 1.2, 2.0),
        Vec3::new(0.0, 0.0, -1.0),
    );

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buffer.get_data_ref())?;
    Ok(())
}
