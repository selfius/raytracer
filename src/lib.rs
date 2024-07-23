pub mod buffer;
mod ray_tracing;
mod vector_math;
mod scene;

use crate::buffer::{Buffer, Point, Rgb};
use crate::vector_math::Vec3;

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 768;
pub const CHANNELS: u8 = 3;

const HORIZONTAL_FOV: f32 = 90.0;


pub fn draw(buffer: &mut Buffer) {
    let camera_position = Vec3::new(0.0, 1.0, 0.0);
    let looking_direction = Vec3::new(0.0, 0.0, -1.0);
    let (in_world_top_left, in_world_pixel_x_offset, in_world_pixel_y_offset) =
        set_up_3d_world(camera_position, looking_direction);

    let scene = scene::create_scene();

    let spec_base_color = Rgb::new(255, 255, 255);

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // mapping between the pixel on the png and the vector looking at its representation
            // on the virtual screen from the pov of the camera
            let camera_to_pixel_direction = in_world_pixel_x_offset * x as f32
                + in_world_pixel_y_offset * y as f32
                + in_world_top_left
                - camera_position;

            if let Some((sphere, distance)) =
                ray_tracing::scene_intersect(&camera_position, &camera_to_pixel_direction, &scene.spheres)
            {
                let mut diffuse_intensity: f32 = 0.0;
                let mut specular_intensity: f32 = 0.0;

                let point_on_sphere =
                    camera_position + (camera_to_pixel_direction.normalize() * distance);
                let normal = (point_on_sphere - sphere.origin).normalize();

                for light in &scene.lights {
                    let light_direction = (light.origin - point_on_sphere).normalize();

                    if let Some((obstructing_sphere, _)) = ray_tracing::scene_intersect(
                        &light.origin,
                        &(point_on_sphere - light.origin),
                        &scene.spheres,
                    ) {
                        if obstructing_sphere != sphere {
                            continue;
                        }
                    }

                    diffuse_intensity += (light_direction * normal).max(0.0) * light.intensity;

                    specular_intensity += (light_direction.reflection(&normal)
                        * (camera_to_pixel_direction.normalize() * -1.0))
                        .max(0.0)
                        .powf(sphere.material.shininess);
                }

                buffer.set(
                    &Point(x, y),
                    &(sphere.material.diffuse_color.clone()
                        * (diffuse_intensity * (sphere.material.albedo.0)).min(1.0)
                        + spec_base_color.clone()
                            * (specular_intensity * sphere.material.albedo.1)),
                );
            };
        }
    }
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
