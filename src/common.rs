#[cfg(test)]
pub mod test {
    
    pub fn cap_float(value: f32) -> f32 {
        (value * 10.0).round() / 10.0
    }
}