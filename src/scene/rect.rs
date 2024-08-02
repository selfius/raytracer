use super::triangle::Triangle;
use crate::vector_math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Rect {
    triangles: Vec<Triangle>,
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

    pub fn find_intersection(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<(f32, Vec3)> {
        let [half, another] = self.as_triangles();
        half.find_intersection(ray_origin, ray_direction)
            .or(another.find_intersection(ray_origin, ray_direction))
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common::test::*;

    #[test]
    fn ray_intersects_rect() {
        let rect = Rect::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );

        let intersection =
            rect.find_intersection(&Vec3::new(0.5, 0.5, 1.0), &Vec3::new(0.0, 0.0, -2.0));

        assert_eq!(Some((1.0, Vec3::new(0.0, 0.0, -1.0))), intersection);
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
