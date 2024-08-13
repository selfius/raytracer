use lib::buffer::{Buffer, Dimensions};
use png::Encoder;
use std::{fs::File, io};

fn main() -> io::Result<()> {
    let output = File::create("test.png")?;
    let mut encoder = Encoder::new(output, lib::WIDTH, lib::HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut buffer = Buffer::new(Dimensions(lib::WIDTH, lib::HEIGHT), lib::CHANNELS);
    buffer.clear();
    lib::draw(&mut buffer);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buffer.get_data_ref())?;
    Ok(())
}
