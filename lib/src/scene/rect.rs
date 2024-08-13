use super::triangle::Triangle;
use super::Surface;

use crate::ray_tracing::Intersection;
use crate::vector_math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Rect {
    triangles: Vec<Triangle>,
}

impl Surface for Rect {
    fn find_intersection(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersection> {
        if let Some((x, y)) = self.find_intesection_coords(ray_origin, ray_direction) {
            let [a, b, _, d] = self.as_vertices();
            let intersection_point = a + (d - a) * x + (b - a) * y;
            if (intersection_point - *ray_origin) * *ray_direction > 0.0 {
                return Some(Intersection {
                    distance: (intersection_point - *ray_origin).magnitude(),
                    normal: self
                        .as_triangles()
                        .iter()
                        .next()
                        .map(|triangle| triangle.normal())
                        .unwrap(),
                    texture_coords: Some((x, y)),
                });
            }
        }
        None
    }

    fn approximate_outside(&self, point_on_surface: Vec3) -> Vec3 {
        point_on_surface + (self.as_triangles()[0].normal() * 1e-6)
    }
}

impl Rect {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3) -> Rect {
        let half = Triangle::new(a, c, b);
        let another = Triangle::new(c, a, d);

        assert!((half.normal() - another.normal()).magnitude() < f32::EPSILON);
        Rect {
            triangles: vec![half, another],
        }
    }

    pub fn find_intesection_coords(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<(f32, f32)> {
        let [half, another] = self.as_triangles();
        half.find_barycentric_intersection(ray_origin, ray_direction)
            .map(|(u, v)| Vec3::new(1.0, 1.0, 0.0) * u + Vec3::new(0.0, 1.0, 0.0) * v)
            .or(another
                .find_barycentric_intersection(ray_origin, ray_direction)
                .map(|(u, v)| {
                    Vec3::new(1.0, 1.0, 0.0)
                        + Vec3::new(-1.0, -1.0, 0.0) * u
                        + Vec3::new(0.0, -1.0, 0.0) * v
                }))
            .map(|vec| {
                (
                    vec * Vec3::new(1.0, 0.0, 0.0),
                    vec * Vec3::new(0.0, 1.0, 0.0),
                )
            })
    }

    fn as_triangles(&self) -> [&Triangle; 2] {
        match &self.triangles[..] {
            [a, b] => [a, b],
            _ => panic!("rectangle must consist of two triangles"),
        }
    }

    fn as_vertices(&self) -> [Vec3; 4] {
        let [half, another] = self.as_triangles();
        let [a, _, b] = half.as_vertices();
        let [c, _, d] = another.as_vertices();
        [a, b, c, d]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::test::*;

    #[test]
    fn ray_intersects_rect() {
        let rect = Rect::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );

        let intersection =
            rect.find_intersection(&Vec3::new(0.5, 0.5, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(
            Some(Intersection {
                distance: 1.0,
                normal: Vec3::new(0.0, -0.0, 1.0),
                texture_coords: Some((0.5, 0.5))
            }),
            intersection
        );
    }

    #[test]
    fn ray_intersect_scaled_rect() {
        let rect = Rect::new(
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(3.0, 0.0, 0.0),
        );

        let intersection =
            rect.find_intersection(&Vec3::new(1.0, 1.0, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(
            Some(Intersection {
                distance: 1.0,
                normal: Vec3::new(0.0, 0.0, 1.0),
                texture_coords: Some((0.0, 0.5))
            }),
            intersection
        );
    }

    #[test]
    fn ray_intersect_at_first_triangle_coords() {
        let rect = Rect::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );

        let intersection =
            rect.find_intesection_coords(&Vec3::new(0.1, 0.9, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(Some((0.1, 0.9)), intersection);
    }

    #[test]
    fn ray_intersect_at_first_second_coords() {
        let rect = Rect::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );

        let intersection = rect
            .find_intesection_coords(&Vec3::new(0.9, 0.1, 1.0), &Vec3::new(0.0, 0.0, -2.0))
            .map(|(x, y)| (cap_float(x), cap_float(y)));

        assert_eq!(Some((0.9, 0.1)), intersection);
    }
}
