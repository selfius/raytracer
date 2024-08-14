use ndarray::Array3;

use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;

use std::path::Path;

use lib::buffer::{Buffer, Dimensions, Point};
use lib::vector_math::Vec3;

fn main() {
    video_rs::init().unwrap();

    let settings = Settings::preset_h264_yuv420p(lib::WIDTH as usize, lib::HEIGHT as usize, false);
    let mut encoder =
        Encoder::new(Path::new("output.mp4"), settings).expect("failed to create encoder");

    let duration: Time = Time::from_nth_of_a_second(24);
    let mut position = Time::zero();
    let look_at = Vec3::new(0.0, 1.0, -7.5);
    let mut angle = 90.0_f32;
    for _i in 0..180 {
        angle += 2.0;
        let (x, z) = (
            f32::cos(angle.to_radians()) * 8.0,
            f32::sin(angle.to_radians()) * 8.0,
        );
        let camera_position = Vec3::new(x, 1.0, z);
        let frame = generate_frame(look_at + camera_position, -camera_position);

        encoder
            .encode(&frame, position)
            .expect("failed to encode frame");

        // Update the current position and add the inter-frame duration to it.
        position = position.aligned_with(duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}

fn generate_frame(camera_position: Vec3, camera_direction: Vec3) -> Array3<u8> {
    let mut buffer = Buffer::new(Dimensions(lib::WIDTH, lib::HEIGHT), lib::CHANNELS);
    let mut frame = Array3::zeros((lib::HEIGHT as usize, lib::WIDTH as usize, 3));

    lib::draw(&mut buffer, camera_position, camera_direction);
    for x in 0..lib::WIDTH {
        for y in 0..lib::HEIGHT {
            let rgb = buffer.get(&Point(x, y)).as_bites();
            let (x, y) = (x as usize, y as usize);
            frame[[y, x, 0]] = rgb.0;
            frame[[y, x, 1]] = rgb.1;
            frame[[y, x, 2]] = rgb.2;
        }
    }
    frame
}
