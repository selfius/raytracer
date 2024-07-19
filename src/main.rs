mod buffer;
mod vector_math;

use buffer::{Buffer, Dimensions, Point, Rgb};
use png::Encoder;
use std::{fs::File, io};
use vector_math::Vec3;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const CHANNELS: u8 = 3;

const HORIZONTAL_FOV: f32 = 90.0;

fn main() -> io::Result<()> {
    let output = File::create("test.png")?;
    let mut encoder = Encoder::new(output, WIDTH, HEIGHT);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut buffer = Buffer::new(Dimensions(WIDTH, HEIGHT), CHANNELS);
    buffer.clear();
    background(&mut buffer);
    draw(&mut buffer);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(buffer.get_data_ref())?;
    Ok(())
}

fn background(buffer: &mut Buffer) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            buffer.set(
                &Point(x, y),
                &Rgb((x * 256 / WIDTH) as u8, 0, (y * 256 / HEIGHT) as u8),
            );
        }
    }
}

fn draw(buffer: &mut Buffer) {
    let camera_position = Vec3::new(0.0, 1.0, 0.0);
    let looking_direction = Vec3::new(0.0, 0.0, -1.0);
    let (in_world_top_left, in_world_pixel_x_offset, in_world_pixel_y_offset) =
        set_up_3d_world(camera_position, looking_direction);

    let sphere = Sphere {
        origin: Vec3::new(3.2, 1.8, -10.0),
        radius: 2.0,
        diffuse_color: Rgb(50, 50, 50),
    };

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // mapping between the pixel on the png and the vector looking at its representation
            // on the virtual screen from the pov of the camera
            let camera_to_pixel_direction = in_world_pixel_x_offset * x as f32
                + in_world_pixel_y_offset * y as f32
                + in_world_top_left
                - camera_position;

            let distance = ray_intersects(&camera_position, &camera_to_pixel_direction, &sphere);
            if distance.is_some() {
                // distance should be in range between 8.0 to 10.0 for our particular sphere
                // lets map that to the shades of grey (the closer the brigther)
                let brightness = 255 - (255.0 * (distance.unwrap() - 8.0) / 2.0) as u8;
                buffer.set(&Point(x, y), &Rgb(brightness, brightness, brightness));
            }
        }
    }
}

struct Sphere {
    origin: Vec3,
    radius: f32,
    diffuse_color: Rgb,
}

fn ray_intersects(ray_origin: &Vec3, ray_direction: &Vec3, sphere: &Sphere) -> Option<f32> {
    let normalized_ray_direction = ray_direction.normalize();
    let ray_origin_to_sphere = sphere.origin - *ray_origin;
    if ray_origin_to_sphere * normalized_ray_direction > 0.0 {
        let center_to_ray = (ray_origin_to_sphere.magnitude().powi(2)
            - (ray_origin_to_sphere * normalized_ray_direction).powi(2))
        .sqrt();
        if center_to_ray < sphere.radius {
            let distance = (normalized_ray_direction
                * (ray_origin_to_sphere * normalized_ray_direction)
                - ray_direction.normalize()
                    * (sphere.radius.powi(2) - center_to_ray.powi(2)).sqrt())
            .magnitude();
            return Option::Some(distance);
        }
    }
    Option::None
}

fn set_up_3d_world(camera_position: Vec3, _looking_direction: Vec3) -> (Vec3, Vec3, Vec3) {
    // TODO: we should take into account looking direction to calculate virtual screen placement
    // and x- and y- offsets. For the time being we assume this direction to be (0,0, -1)
    let vertical_fov = (HEIGHT as f32 * (HORIZONTAL_FOV / 2.0).to_radians().tan() / WIDTH as f32)
        .atan()
        .to_degrees()
        * 2.0;

    // virtual screen size in world coordinates
    let in_world_screen_width = 2.0 * (HORIZONTAL_FOV / 2.0).to_radians().tan();
    let in_world_screen_height = 2.0 * (vertical_fov / 2.0).to_radians().tan();

    // size of a pixel in the output translated to world coordinates
    let in_world_pixel_x_offset = Vec3::new(in_world_screen_width / (WIDTH as f32), 0.0, 0.0);
    let in_world_pixel_y_offset = Vec3::new(0.0, -in_world_screen_height / (HEIGHT as f32), 0.0);

    // top left of the virtual screen
    let in_world_top_left = camera_position
        + Vec3::new(
            -in_world_screen_width / 2.0,
            in_world_screen_height / 2.0,
            -1.0,
        );
    (
        in_world_top_left,
        in_world_pixel_x_offset,
        in_world_pixel_y_offset,
    )
}
