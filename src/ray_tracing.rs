use super::Sphere;
use crate::vector_math::Vec3;

pub fn scene_intersect<'a>(
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
mod tests {
    use super::*;
    use crate::{buffer::Rgb, Material, Sphere};

    #[test]
    fn scene_intersect_picks_closest() {
        let spheres = vec![
            Sphere {
                origin: Vec3::new(3.0, 0.0, 0.0),
                radius: 1.0,
                material: Material {
                    diffuse_color: Rgb(255, 0, 0),
                    shininess: 0.0,
                    albedo: (0.5, 0.5),
                },
            },
            Sphere {
                origin: Vec3::new(4.0, 0.0, 0.0),
                radius: 1.5,
                material: Material {
                    diffuse_color: Rgb(0, 0, 0),
                    shininess: 0.0,
                    albedo: (0.5, 0.5),
                },
            },
            Sphere {
                origin: Vec3::new(-10.0, 0.0, 0.0),
                radius: 1.5,
                material: Material {
                    diffuse_color: Rgb(0, 0, 255),
                    shininess: 0.0,
                    albedo: (0.5, 0.5),
                },
            },
        ];

        let intersection = scene_intersect(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &spheres,
        );

        assert_eq!(
            intersection.unwrap().0.material.diffuse_color,
            Rgb(255, 0, 0)
        );
    }

    #[test]
    fn ray_intersects_test() {
        let sphere = Sphere {
            origin: Vec3::new(3.0, 0.0, 0.0),
            radius: 2.0,
            material: Material {
                diffuse_color: Rgb(255, 0, 0),
                shininess: 0.0,
                albedo: (0.5, 0.5),
            },
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
            material: Material {
                diffuse_color: Rgb(255, 0, 0),
                shininess: 0.0,
                albedo: (0.5, 0.5),
            },
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
            material: Material {
                diffuse_color: Rgb(255, 0, 0),
                shininess: 0.0,
                albedo: (0.5, 0.5),
            },
        };

        let distance = ray_intersects(
            &Vec3::new(0.0, 0.0, 0.0),
            &Vec3::new(1.0, 0.0, 0.0),
            &sphere,
        );

        assert_eq!(distance, None);
    }
}
