use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    values: [f32; 3],
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, v2: Self) -> Self {
        let mut new_values = [0.0; 3];
        for (idx, (v1_i, v2_i)) in self.values.iter().zip(v2.values).enumerate() {
            new_values[idx] = v1_i + v2_i;
        }
        Self { values: new_values }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(mut self, rhs: f32) -> Self {
        for i in &mut self.values {
            *i *= rhs;
        }
        self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f32;
    fn mul(self, rhs: Vec3) -> f32 {
        let mut product = 0f32;
        for (v1_i, v2_i) in self.values.iter().zip(rhs.values.iter()) {
            product += v1_i * v2_i
        }
        product
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self {
        self + -rhs
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { values: [x, y, z] }
    }

    pub fn as_coords(&self) -> (f32, f32, f32) {
        match self.values {
            [x, y, z] => (x, y, z),
        }
    }

    pub fn magnitude(&self) -> f32 {
        let mut result = 0.0;
        for i in self.values {
            result += i * i;
        }
        f32::sqrt(result)
    }

    pub fn normalize(self) -> Vec3 {
        let magnitude = self.magnitude();
        self * (1.0 / magnitude)
    }

    pub fn reflection(self, normal: &Vec3) -> Vec3 {
        let normal = normal.clone().normalize();
        let normalized_self = self.normalize();
        normal * (2.0 * (normalized_self * normal)) - normalized_self
    }

    pub fn refraction(
        self,
        normal: &Vec3,
        current_refraction_index: f32,
        next_refraction_index: f32,
    ) -> Vec3 {
        let normal = normal.normalize();
        let normalized_self = self.normalize();
        let r = current_refraction_index / next_refraction_index;
        let c = -normal * normalized_self;
        (self * r + normal * (r * c - f32::sqrt(1.0 - r.powi(2) * (1.0 - c.powi(2))))).normalize()
    }

    pub fn cross_product(self, rhs: &Vec3) -> Vec3 {
        let [x1, y1, z1] = self.values;
        let [x2, y2, z2] = rhs.values;

        Vec3::new(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;
    use crate::common::test::*;

    #[test]
    fn addition() {
        let result = Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 5.0, 6.0);
        let expected = Vec3::new(5.0, 7.0, 9.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn substraction() {
        let result = Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 5.0, 6.0);
        let expected = Vec3::new(-3.0, -3.0, -3.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn multiplication_by_scalar() {
        let result = Vec3::new(1.0, 2.0, 3.0) * 2.0;
        let expected = Vec3::new(2.0, 4.0, 6.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn dot_product() {
        let result = Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 5.0, 6.0);
        let expected = 32.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn magnitude() {
        let result = Vec3::new(-10.0, 0.0, 0.0).magnitude();
        let expected = 10.0;
        assert_eq!(expected, result);
    }

    #[test]
    fn normalization() {
        let result = Vec3::new(-10.0, 0.0, 0.0).normalize();
        let expected = Vec3::new(-1.0, 0.0, 0.0);
        assert_eq!(expected, result);
    }

    #[test]
    fn reflection() {
        let reflected_vector = Vec3::new(1.0, 1.0, 0.0).reflection(&Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(reflected_vector, Vec3::new(-1.0, 1.0, 0.0).normalize());
    }

    #[test]
    fn refraction() {
        let refracted_vector =
            Vec3::new(1.0, -1.0, 0.0).refraction(&Vec3::new(0.0, 1.0, 0.0), 1.0, 1.1);
        let [x, y, z] = refracted_vector.values;
        assert_eq!(
            Vec3 {
                values: [cap_float(x), cap_float(y), cap_float(z)]
            },
            Vec3::new(0.7, -0.8, 0.0)
        );
    }

    #[test]
    fn cross_product() {
        let cross_product = Vec3::new(1.0, 0.0, 0.0).cross_product(&Vec3::new(0.0, 1.0, 0.0));

        assert_eq!(cross_product, Vec3::new(0.0, 0.0, 1.0));
    }
}
