use ndarray::Array3;

use video_rs::encode::{Encoder, Settings};
use video_rs::time::Time;

use std::path::Path;

fn main() {
    video_rs::init().unwrap();

    let settings = Settings::preset_h264_yuv420p(1280, 720, false);
    let mut encoder =
        Encoder::new(Path::new("output.mp4"), settings).expect("failed to create encoder");

    let duration: Time = Time::from_nth_of_a_second(24);
    let mut position = Time::zero();
    for i in 0..256 {
        // This will create a smooth rainbow animation video!
        let frame = gradient(i as f32 / 256.0);

        encoder
            .encode(&frame, position)
            .expect("failed to encode frame");

        // Update the current position and add the inter-frame duration to it.
        position = position.aligned_with(duration).add();
    }

    encoder.finish().expect("failed to finish encoder");
}

fn gradient(_frame_idx: f32) -> Array3<u8> {
    let mut frame = Array3::from_shape_fn((720, 1280, 3), |(_y, _x, _c)| 0u8);
    for x in 0..1280 {
        for y in 0..720 {
            frame[[y, x, 0]] = (x as f32 / 1280.0 * 255.0) as u8;
            frame[[y, x, 1]] = (y as f32 / 1280.0 * 255.0) as u8;
        }
    }
    frame
}
