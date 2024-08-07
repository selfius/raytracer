use crate::vector_math::Vec3;

use super::Surface;
use crate::ray_tracing::Intersection;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
}

fn get_texture_coords(origin: &Vec3, point_on_sphere: &Vec3) -> (f32, f32) {
    let origin_to_point = (*point_on_sphere - *origin).normalize();
    const X0: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);
    let mut x = (origin_to_point * X0 * -1.0 + 1.0) / 4.0; //mapping cos to 0..0.5 from 1..-1
    if origin_to_point * FORWARD < 0.0 {
        x = 1.0 - x;
    }

    const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    let y = (origin_to_point * UP * -1.0 + 1.0) / 2.0; //mapping to 0..1 from 1..-1
    (x, y)
}

impl Sphere {}

impl Surface for Sphere {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection> {
        let ray_direction = ray_direction.normalize();
        let ray_origin_to_sphere = self.origin - *ray_origin;
        let center_to_ray_square = ray_origin_to_sphere.magnitude().powi(2)
            - (ray_origin_to_sphere * ray_direction).powi(2);
        if center_to_ray_square <= self.radius.powi(2) {
            let distance_to_intersection_with_ray = ray_origin_to_sphere * ray_direction;
            let delta = (self.radius.powi(2) - center_to_ray_square).sqrt();
            if distance_to_intersection_with_ray - delta >= 0.0 {
                let distance = distance_to_intersection_with_ray - delta;
                let point_on_sphere = *ray_origin + (ray_direction * distance);
                return Some(Intersection {
                    distance,
                    normal: point_on_sphere - self.origin,
                    texture_coords: Some(get_texture_coords(&self.origin, &point_on_sphere)),
                });
            } else if distance_to_intersection_with_ray + delta >= 0.0 {
                let distance = distance_to_intersection_with_ray + delta;
                let point_on_sphere = *ray_origin + (ray_direction * distance);
                return Some(Intersection {
                    distance,
                    normal: point_on_sphere - self.origin,
                    texture_coords: Some(get_texture_coords(&self.origin, &point_on_sphere)),
                });
            }
        }
        Option::None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersects_test() {
        let sphere = Sphere {
            origin: Vec3::new(3.0, 0.0, 0.0),
            radius: 2.0,
        };

        let Intersection { distance, .. } = sphere
            .find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
            .unwrap();

        assert_eq!(distance, 1.0);
    }

    #[test]
    fn ray_intersects_with_sphere_behind_camera() {
        let sphere = Sphere {
            origin: Vec3::new(-3.0, 0.0, 0.0),
            radius: 2.0,
        };

        let result = sphere.find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0));

        assert_eq!(result, None);
    }

    #[test]
    fn ray_intersects_with_camera_inside_sphere() {
        let sphere = Sphere {
            origin: Vec3::new(-1.0, 0.0, 0.0),
            radius: 2.0,
        };

        let Intersection { distance, .. } = sphere
            .find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 0.0, 0.0))
            .unwrap();

        assert_eq!(distance, 1.0);
    }
}
