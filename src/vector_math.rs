use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy)]
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
        self + (rhs * -1.0)
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
}
