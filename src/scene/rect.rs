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

    pub fn normal(&self) -> Vec3 {
        self.triangles
            .iter()
            .next()
            .map(Triangle::normal)
            .expect("Rect must consist of two triangles")
    }

    pub fn find_intersection(
        &self,
        ray_origin: &Vec3,
        ray_direction: &Vec3,
    ) -> Option<(f32, Vec3)> {
        if let [half, another] = &self.triangles[..] {
            return half
                .find_intersection(ray_origin, ray_direction)
                .or(another.find_intersection(ray_origin, ray_direction));
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

        assert_eq!(Some((1.0, Vec3::new(0.0, 0.0, 1.0))), intersection);
    }
}
