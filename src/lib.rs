pub mod buffer;
mod common;
mod ray_tracing;
mod scene;
mod vector_math;

use std::ptr;
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;

use scene::{Object, Scene};

use crate::buffer::{Buffer, Point, Rgb};
use crate::vector_math::Vec3;

pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 768;
pub const CHANNELS: u8 = 3;
pub const THREADS: usize = 16;

const HORIZONTAL_FOV: f32 = 90.0;

pub fn draw(buffer: &mut Buffer) {
    let camera_position = Vec3::new(0.0, 1.0, 0.0);
    let looking_direction = Vec3::new(0.0, 0.0, -1.0);
    let (in_world_top_left, in_world_pixel_x_offset, in_world_pixel_y_offset) =
        set_up_3d_world(camera_position, looking_direction);

    let scene = Arc::new(scene::create_scene());

    let pool = ThreadPool::new(THREADS);
    let (tx, rx) = mpsc::channel();

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // mapping between the pixel on the png and the vector looking at its representation
            // on the virtual screen from the pov of the camera
            let camera_to_pixel_direction = in_world_pixel_x_offset * x as f32
                + in_world_pixel_y_offset * y as f32
                + in_world_top_left
                - camera_position;

            let tx = tx.clone();
            let scene = Arc::clone(&scene);

            pool.execute(move || {
                tx.send((
                    x,
                    y,
                    cast_ray(
                        &camera_position,
                        &camera_to_pixel_direction,
                        &scene,
                        0,
                        None,
                    ),
                ))
                .unwrap();
            });
        }
    }
    drop(tx);
    for (x, y, color) in rx {
        buffer.set(&Point(x, y), &color);
    }
}

fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    scene: &Scene,
    bounce_count: u8,
    current_medium: Option<&Object>,
) -> Rgb {
    if let Some((object, intersection)) =
        ray_tracing::scene_intersect(ray_origin, ray_direction, &scene)
    {
        let mut diffuse_intensity: f32 = 0.0;
        let mut specular_intensity: f32 = 0.0;

        let ray_direction = ray_direction.normalize();

        let point_on_object = *ray_origin + (ray_direction * intersection.distance);
        let normal = intersection.normal.normalize();

        for light in &scene.lights {
            let light_direction = (light.origin - point_on_object).normalize();

            if let Some((obstructing_object, _)) = ray_tracing::scene_intersect(
                &light.origin,
                &(point_on_object - light.origin),
                &scene,
            ) {
                if !ptr::eq(obstructing_object, object) {
                    continue;
                }
            }

            diffuse_intensity += (light_direction * normal).max(0.0) * light.intensity;

            specular_intensity += (light_direction.reflection(&normal) * -ray_direction)
                .max(0.0)
                .powf(object.material.shininess);
        }

        let mut reflection_component = Rgb::new(0, 0, 0);
        let mut refraction_component = Rgb::new(0, 0, 0);

        if bounce_count < BOUNCE_LIMIT {
            if object.material.albedo.2 > 0.0 {
                let reflection_direction = -ray_direction.reflection(&normal);

                let rounding_error_compensation = if reflection_direction * normal > 0.0 {
                    -ROUNDING_COMPENSATION
                } else {
                    ROUNDING_COMPENSATION
                };

                let reflection_origin = *ray_origin
                    + (ray_direction * (intersection.distance + rounding_error_compensation));
                reflection_component = cast_ray(
                    &reflection_origin,
                    &reflection_direction,
                    scene,
                    bounce_count + 1,
                    None,
                ) * object.material.albedo.2;
            }
            if object.material.albedo.3 > 0.0 {
                let next_refraction_medium = match current_medium {
                    None => Some(object),
                    Some(_) => None,
                };

                let normal = normal * current_medium.map_or(1.0, |_| -1.0);

                let get_reflective_index_from_sphere =
                    |sphere: &Object| sphere.material.refractive_index;

                let current_index = current_medium.map_or(1.0, get_reflective_index_from_sphere);
                let next_index =
                    next_refraction_medium.map_or(1.0, get_reflective_index_from_sphere);

                let refraction_direciton =
                    ray_direction.refraction(&normal, current_index, next_index);

                let rounding_error_compensation = if refraction_direciton * normal > 0.0 {
                    -ROUNDING_COMPENSATION
                } else {
                    ROUNDING_COMPENSATION
                };

                let refraction_origin = *ray_origin
                    + (ray_direction * (intersection.distance + rounding_error_compensation));
                refraction_component = cast_ray(
                    &refraction_origin,
                    &refraction_direciton,
                    scene,
                    bounce_count + 1,
                    next_refraction_medium,
                ) * object.material.albedo.3;
            }
        }

        return object.material.diffuse_color.clone()
            * (diffuse_intensity * (object.material.albedo.0)).min(1.0)
            + SPEC_BASE_COLOR.clone() * (specular_intensity * object.material.albedo.1)
            + reflection_component
            + refraction_component;
    }
    BACKGROUND_COLOR
}

const BACKGROUND_COLOR: Rgb = Rgb::new(134, 75, 165);

const SPEC_BASE_COLOR: Rgb = Rgb::new(255, 255, 255);

const BOUNCE_LIMIT: u8 = 4;

const ROUNDING_COMPENSATION: f32 = 0.01;

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
