use core::f32;

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
    const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    let x = origin_to_point * X;
    let z = origin_to_point * Z;
    let x = if x == 0.0 && z == 0.0 {
        0.0
    } else {
        let xz = Vec3::new(x, 0.0, z).normalize();
        let mut x = ((xz * X + 1.0) - f32::EPSILON) / 4.0; //mapping to 0..0.5 from 1..-1
        if xz * Z < 0.0 {
            x = 1.0 - x;
        }
        x
    };

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

    fn approximate_inside(&self, point_on_surface: Vec3) -> Vec3 {
        let origin_to_point = point_on_surface - self.origin;

        let next_down = f32::from_bits(self.radius.to_bits() - LAST_PLACE_UNIT_ERROR_MARGIN);
        self.origin + (origin_to_point.normalize() * next_down)
    }

    fn approximate_outside(&self, point_on_surface: Vec3) -> Vec3 {
        let origin_to_point = point_on_surface - self.origin;

        let next_up = f32::from_bits(self.radius.to_bits() + LAST_PLACE_UNIT_ERROR_MARGIN);
        self.origin + (origin_to_point.normalize() * next_up)
    }
}

const LAST_PLACE_UNIT_ERROR_MARGIN: u32 = 8;

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::test::*;

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

    const ORIGIN: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    #[test]
    fn texture_coord_top_left() {
        let coords = get_texture_coords(&ORIGIN, &Vec3::new(0.0, 1.0, 0.0));

        assert_eq!((0.0, 0.0), coords);
    }

    #[test]
    fn texture_coord_bottom_right() {
        let point_on_sphere = Vec3::new(-0.1, -1.0, -0.001).normalize();
        let (x, y) = get_texture_coords(&ORIGIN, &point_on_sphere);

        assert_eq!((1.0, 1.0), (cap_float(x), cap_float(y)));
    }
}
