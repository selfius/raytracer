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
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { values: [x, y, z] }
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
        let normal_copy = normal.clone().normalize();
        let normalized_self = self.normalize();
        normal_copy * (2.0 * (normalized_self * normal_copy)) - normalized_self
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

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
}
