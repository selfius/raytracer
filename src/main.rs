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

    let spheres = vec![
        Sphere {
            origin: Vec3::new(3.2, 1.8, -10.0),
            radius: 2.0,
            diffuse_color: Rgb(250, 50, 50),
        },
        Sphere {
            origin: Vec3::new(4.0, 0.8, -7.0),
            radius: 1.0, // slighly close the camera then the first shpere
            diffuse_color: Rgb(50, 250, 50),
        },
        Sphere {
            origin: Vec3::new(-1.0, 0.2, -3.0),
            radius: 1.0, // slighly close the camera then the first shpere
            diffuse_color: Rgb(50, 50, 250),
        },
        Sphere {
            origin: Vec3::new(-2.2, 4.5, -16.0),
            radius: 4.0, // slighly close the camera then the first shpere
            diffuse_color: Rgb(200, 150, 70),
        },
    ];

    let lights = vec![
        Light {
            origin: Vec3::new(-4.0, 40.0, 70.0),
            intensity: 0.4,
        },
        Light {
            origin: Vec3::new(5.0, 0.5, -4.0),
            intensity: 0.8,
        },
    ];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            // mapping between the pixel on the png and the vector looking at its representation
            // on the virtual screen from the pov of the camera
            let camera_to_pixel_direction = in_world_pixel_x_offset * x as f32
                + in_world_pixel_y_offset * y as f32
                + in_world_top_left
                - camera_position;

            match scene_intersect(&camera_position, &camera_to_pixel_direction, &spheres) {
                Some((sphere, distance)) => {
                    let mut diffuse_intensity: f32 = 0.0;

                    let point_on_sphere =
                        camera_position + (camera_to_pixel_direction.normalize() * distance);
                    let normal = (point_on_sphere - sphere.origin).normalize();

                    for light in &lights {
                        let light_direction = (light.origin - point_on_sphere).normalize();
                        let obstructing_sphere = scene_intersect(
                            &light.origin,
                            &(point_on_sphere - light.origin),
                            &spheres,
                        );
                        if obstructing_sphere.is_some() && obstructing_sphere.unwrap().0 != sphere {
                            continue;
                        }

                        diffuse_intensity += (light_direction * normal).max(0.0) * light.intensity;
                    }

                    buffer.set(
                        &Point(x, y),
                        &(sphere.diffuse_color.clone() * diffuse_intensity.min(1.0)),
                    );
                }
                _ => (),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Sphere {
    origin: Vec3,
    radius: f32,
    diffuse_color: Rgb,
}

struct Light {
    origin: Vec3,
    intensity: f32,
}

fn scene_intersect<'a>(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    spheres: &'a Vec<Sphere>,
) -> Option<(&'a Sphere, f32)> {
    let mut closest = f32::MAX;
    let mut closest_sphere: Option<(&Sphere, f32)> = None;
    for sphere in spheres {
        closest_sphere = match ray_intersects(&ray_origin, &ray_direction, &sphere) {
            Some(distance) => {
                if distance < closest && distance > 0.0 {
                    closest = distance;
                    Some((sphere, distance))
                } else {
                    closest_sphere
                }
            }
            None => closest_sphere,
        }
    }
    closest_sphere
}
#[cfg(test)]
mod scene_intersect_test {
    use super::*;

    #[test]
    fn scene_intersect_picks_closest() {
        let spheres = vec![
            Sphere {
                origin: Vec3::new(3.0, 0.0, 0.0),
                radius: 1.0,
                diffuse_color: Rgb(255, 0, 0),
            },
            Sphere {
                origin: Vec3::new(4.0, 0.0, 0.0),
                radius: 1.5,
                diffuse_color: Rgb(0, 0, 0),
            },
            Sphere {
                origin: Vec3::new(-10.0, 0.0, 0.0),
                radius: 1.5,
                diffuse_color: Rgb(0, 0, 255),
            },
        ];

        let intersection = scene_intersect(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &spheres,
        );

        assert_eq!(intersection.unwrap().0.diffuse_color, Rgb(255, 0, 0));
    }
}

fn ray_intersects(ray_origin: &Vec3, ray_direction: &Vec3, sphere: &Sphere) -> Option<f32> {
    let normalized_ray_direction = ray_direction.normalize();
    let ray_origin_to_sphere = sphere.origin - *ray_origin;
    if ray_origin_to_sphere.magnitude() < sphere.radius {
        return None;
    }
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

#[cfg(test)]
mod ray_intersects_test {
    use super::*;

    #[test]
    fn ray_intersects_test() {
        let sphere = Sphere {
            origin: Vec3::new(3.0, 0.0, 0.0),
            radius: 2.0,
            diffuse_color: Rgb(255, 0, 0),
        };

        let distance = ray_intersects(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &sphere,
        );

        assert_eq!(distance, Some(1.0));
    }

    #[test]
    fn ray_intersects_with_sphere_behind_camera() {
        let sphere = Sphere {
            origin: Vec3::new(-3.0, 0.0, 0.0),
            radius: 2.0,
            diffuse_color: Rgb(255, 0, 0),
        };

        let distance = ray_intersects(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &sphere,
        );

        assert_eq!(distance, None);
    }

    #[test]
    fn ray_intersects_with_camera_inside_sphere() {
        let sphere = Sphere {
            origin: Vec3::new(1.0, 1.0, 0.0),
            radius: 2.0,
            diffuse_color: Rgb(255, 0, 0),
        };

        let distance = ray_intersects(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &sphere,
        );

        assert_eq!(distance, None);
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
