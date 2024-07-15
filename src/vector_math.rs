#[derive(Debug, Clone)]
pub struct Vec3 {
    values: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { values: [x, y, z] }
    }

    pub fn add(self, v2: &Vec3) -> Vec3 {
        let mut new_values = [0.0; 3];
        for (idx, (v1_i, v2_i)) in self.values.iter().zip(v2.values).enumerate() {
            new_values[idx] = v1_i + v2_i;
        }
        Vec3 { values: new_values }
    }

    pub fn dot_product(&self, v2: &Vec3) -> f32 {
        let mut product = 0f32;
        for (v1_i, v2_i) in self.values.iter().zip(v2.values.iter()) {
            product += v1_i * v2_i
        }
        product
    }

    pub fn multiply(mut self, scalar: f32) -> Vec3 {
        for i in &mut self.values {
            *i *= scalar;
        }
        self
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
        self.multiply(1.0 / magnitude)
    }
}
