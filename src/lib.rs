pub mod buffer;
mod ray_tracing;
mod scene;
mod vector_math;

use scene::Scene;

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

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // mapping between the pixel on the png and the vector looking at its representation
            // on the virtual screen from the pov of the camera
            let camera_to_pixel_direction = in_world_pixel_x_offset * x as f32
                + in_world_pixel_y_offset * y as f32
                + in_world_top_left
                - camera_position;

            buffer.set(
                &Point(x, y),
                &cast_ray(&camera_position, &camera_to_pixel_direction, &scene, 0),
            );
        }
    }
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, scene: &Scene, bounce_count: u8) -> Rgb {
    if let Some((sphere, distance)) =
        ray_tracing::scene_intersect(ray_origin, ray_direction, &scene.spheres)
    {
        let mut diffuse_intensity: f32 = 0.0;
        let mut specular_intensity: f32 = 0.0;

        let ray_direction = ray_direction.normalize();

        let point_on_sphere = *ray_origin + (ray_direction * distance);
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
                * -ray_direction.normalize())
            .max(0.0)
            .powf(sphere.material.shininess);
        }

        let mut reflection_component = Rgb::new(0, 0, 0);

        if bounce_count < BOUNCE_LIMIT && sphere.material.albedo.2 > 0.0 {
            let camera_ray_bounce = -ray_direction.reflection(&normal);

            let point_outside_sphere =
                *ray_origin + (ray_direction.normalize() * (distance - ROUNDING_COMPENSATION));

            reflection_component = cast_ray(
                &point_outside_sphere,
                &camera_ray_bounce,
                scene,
                bounce_count + 1,
            ) * sphere.material.albedo.2;
        }

        return sphere.material.diffuse_color.clone()
            * (diffuse_intensity * (sphere.material.albedo.0)).min(1.0)
            + SPEC_BASE_COLOR.clone() * (specular_intensity * sphere.material.albedo.1)
            + reflection_component;
    }
    BACKGROUND_COLOR
}

const BACKGROUND_COLOR: Rgb = Rgb::new(134, 75, 165);

const SPEC_BASE_COLOR: Rgb = Rgb::new(255, 255, 255);

const BOUNCE_LIMIT: u8 = 4;

const ROUNDING_COMPENSATION: f32 = 0.001;

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
