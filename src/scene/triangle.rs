use super::Surface;

use crate::ray_tracing::Intersection;
use crate::vector_math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Triangle {
    vertices: Vec<Vec3>,
    normal: Vec3,
}
impl Surface for Triangle {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection> {
        let [a, b, c] = self.as_vertices();
        if let Some((u, v)) = self.find_barycentric_intersection(ray_origin, ray_direction) {
            let point_on_triangle = a + (b - a) * u + (c - a) * v;
            let ray_origin_to_intersection = point_on_triangle - *ray_origin;
            if ray_origin_to_intersection * *ray_direction > 0.0 {
                return Some(Intersection {
                    distance: (point_on_triangle - *ray_origin).magnitude(),
                    normal: self.normal,
                    local_coords: None,
                });
            }
        }
        None
    }
}

impl Triangle {
    pub fn new(v1: Vec3, v2: Vec3, v3: Vec3) -> Triangle {
        Triangle {
            vertices: vec![v1, v2, v3],
            normal: (v2 - v1).cross_product(&(v3 - v2)).normalize(),
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn find_barycentric_intersection(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<(f32, f32)> {
        let ray_direction = ray_direction.normalize();
        let direction_cross_normal = ray_direction * self.normal;
        if (direction_cross_normal < f32::EPSILON) && (direction_cross_normal > -f32::EPSILON) {
            //ray is parallel to the plane of the triangle
            return None;
        }

        let [a, b, c] = self.as_vertices();

        let e1 = b - a;
        let e2 = c - a;

        let ray_cross_e2 = ray_direction.cross_product(&e2);
        let det = e1 * ray_cross_e2;

        if det > -f32::EPSILON && det < f32::EPSILON {
            return None; // This ray is parallel to this triangle.
        }

        let inv_det = 1.0 / det;
        let s = *ray_origin - a;
        let u = inv_det * (s * ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let s_cross_e1 = s.cross_product(&e1);
        let v = inv_det * (ray_direction * s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        Some((u, v))
    }

    pub fn find_intersection(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<Intersection> {
        let [a, b, c] = self.as_vertices();
        if let Some((u, v)) = self.find_barycentric_intersection(ray_origin, ray_direction) {
            let point_on_triangle = a + (b - a) * u + (c - a) * v;
            let ray_origin_to_intersection = point_on_triangle - *ray_origin;
            if ray_origin_to_intersection * *ray_direction > 0.0 {
                return Some(Intersection {
                    distance: (point_on_triangle - *ray_origin).magnitude(),
                    normal: self.normal,
                    local_coords: None,
                });
            }
        }
        None
    }

    pub fn as_vertices(&self) -> [Vec3; 3] {
        match self.vertices[..] {
            [a, b, c] => [a, b, c],
            _ => panic!("triangles have 3 vertices"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_intersects_triangle_at_barycentric_coordinates() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection = triangle
            .find_barycentric_intersection(&Vec3::new(0.5, 0.5, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(Some((0.5, 0.5)), intersection);
    }

    #[test]
    fn ray_intersects_triangle() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection =
            triangle.find_intersection(&Vec3::new(0.5, 0.5, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(
            Some(Intersection {
                distance: 1.0,
                normal: Vec3::new(0.0, 0.0, 1.0),
                local_coords: None
            }),
            intersection
        );
    }

    #[test]
    fn ray_intersects_triangle_backside() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection =
            triangle.find_intersection(&Vec3::new(0.5, 0.5, -2.0), &Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(
            Some(Intersection {
                distance: 2.0,
                normal: Vec3::new(0.0, 0.0, 1.0),
                local_coords: None
            }),
            intersection
        );
    }

    #[test]
    fn ray_missed_triangle() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection =
            triangle.find_intersection(&Vec3::new(2.0, 2.0, -1.0), &Vec3::new(0.0, 0.0, 1.0));

        assert_eq!(None, intersection);
    }

    #[test]
    fn ray_parallel_to_triangle() {
        let triangle = Triangle::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection =
            triangle.find_intersection(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(1.0, 1.0, 0.0));

        assert_eq!(None, intersection);
    }
}
