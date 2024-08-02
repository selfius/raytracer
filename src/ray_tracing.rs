use crate::scene::{Object, Scene};
use crate::vector_math::Vec3;

pub fn scene_intersect<'a>(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    scene: &'a Scene,
) -> Option<(&'a Object, Intersection)> {
    let mut closest = f32::MAX;
    let mut closest_object: Option<(&Object, Intersection)> = None;
    for object in &scene.objects {
        closest_object = match object
            .surface
            .find_intersection(&ray_origin, &ray_direction)
        {
            Some(intersection) => {
                if intersection.distance < closest && intersection.distance > 0.0 {
                    closest = intersection.distance;
                    Some((object, intersection))
                } else {
                    closest_object
                }
            }
            None => closest_object,
        }
    }
    closest_object
}

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub distance: f32,
    pub normal: Vec3,
    pub local_coords: Option<(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::Rgb;
    use crate::scene::{sphere::Sphere, Material, Surface};

    #[test]
    fn scene_intersect_picks_closest() {
        let scene = Scene {
            objects: vec![
                Object {
                    surface: Surface::Sphere(Sphere {
                        origin: Vec3::new(3.0, 0.0, 0.0),
                        radius: 1.0,
                    }),
                    material: &RED,
                },
                Object {
                    surface: Surface::Sphere(Sphere {
                        origin: Vec3::new(4.0, 0.0, 0.0),
                        radius: 1.5,
                    }),
                    material: &BLACK,
                },
                Object {
                    surface: Surface::Sphere(Sphere {
                        origin: Vec3::new(-10.0, 0.0, 0.0),
                        radius: 1.5,
                    }),

                    material: &BLACK,
                },
            ],
            lights: vec![],
        };

        let intersection =
            scene_intersect(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0), &scene);

        assert_eq!(
            intersection.unwrap().0.material.diffuse_color,
            Rgb::new(255, 0, 0)
        );
    }

    const RED: Material = Material {
        diffuse_color: Rgb::new(255, 0, 0),
        shininess: 0.0,
        albedo: (0.5, 0.5, 0.0, 0.0),
        refractive_index: 0.0,
    };

    const BLACK: Material = Material {
        diffuse_color: Rgb::new(0, 0, 0),
        shininess: 0.0,
        albedo: (0.5, 0.5, 0.0, 0.0),
        refractive_index: 0.0,
    };
}
