pub mod buffer;
mod common;
mod ray_tracing;
mod scene;
mod vector_math;

use common::DEBUG_PINK;
use ray_tracing::Intersection;
use scene::sphere::Sphere;
use std::ptr;
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;

use scene::{Object, Scene, Surface};

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

                let reflection_origin = *ray_origin + (ray_direction * intersection.distance);

                let reflection_origin = object.surface.approximate_outside(reflection_origin);

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

                let get_reflective_index = |object: &Object| object.material.refractive_index;

                let current_index = current_medium.map_or(1.0, get_reflective_index);
                let next_index = next_refraction_medium.map_or(1.0, get_reflective_index);

                let refraction_direciton =
                    ray_direction.refraction(&normal, current_index, next_index);

                let refraction_origin = *ray_origin + (ray_direction * (intersection.distance));

                let refraction_origin = match current_medium {
                    Some(object) => object.surface.approximate_outside(refraction_origin),
                    None => object.surface.approximate_inside(refraction_origin),
                };

                refraction_component = cast_ray(
                    &refraction_origin,
                    &refraction_direciton,
                    scene,
                    bounce_count + 1,
                    next_refraction_medium,
                ) * object.material.albedo.3;
            }
        }

        return object
            .material
            .diffuse_color
            .get(intersection.texture_coords)
            .clone()
            * (diffuse_intensity * (object.material.albedo.0)).min(1.0)
            + SPEC_BASE_COLOR.clone() * (specular_intensity * object.material.albedo.1)
            + reflection_component
            + refraction_component;
    }
    get_sky_color(ray_direction, scene)
}

fn get_sky_color(ray_direction: &Vec3, scene: &Scene) -> Rgb {
    const SKY_SPHERE: Sphere = Sphere {
        origin: Vec3::new(0.0, 0.0, 0.0),
        radius: 1.0,
    };
    const WORLD_ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    if let Some(Intersection { texture_coords, .. }) =
        SKY_SPHERE.find_intersection(&WORLD_ORIGIN, ray_direction)
    {
        return scene.sky_sphere.diffuse_color.get(texture_coords);
    }
    DEBUG_PINK
}

const SPEC_BASE_COLOR: Rgb = Rgb::new(255, 255, 255);

const BOUNCE_LIMIT: u8 = 4;

fn set_up_3d_world(camera_position: Vec3, looking_direction: Vec3) -> (Vec3, Vec3, Vec3) {
    let vertical_fov = (HEIGHT as f32 * (HORIZONTAL_FOV / 2.0).to_radians().tan() / WIDTH as f32)
        .atan()
        .to_degrees()
        * 2.0;

    // virtual screen size in world coordinates
    let in_world_screen_width = 2.0 * (HORIZONTAL_FOV / 2.0).to_radians().tan();
    let in_world_screen_height = 2.0 * (vertical_fov / 2.0).to_radians().tan();

    let camera_z = -looking_direction.normalize();

    let looking_direction = looking_direction.as_coords();
    let camera_x = Vec3::new(-looking_direction.2, 0.0, looking_direction.0).normalize();
    let camera_y = camera_z.cross_product(&camera_x);

    // size of a pixel in the output translated to world coordinates
    let in_world_pixel_x_offset = camera_x * (in_world_screen_width / (WIDTH as f32));
    let in_world_pixel_y_offset = camera_y * (-in_world_screen_height / (HEIGHT as f32));

    // top left of the virtual screen
    let in_world_top_left = camera_position - camera_x * (in_world_screen_width / 2.0)
        + camera_y * (in_world_screen_height / 2.0)
        - camera_z;

    (
        in_world_top_left,
        in_world_pixel_x_offset,
        in_world_pixel_y_offset,
    )
}
