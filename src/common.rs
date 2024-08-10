use crate::buffer::Rgb;

pub const DEBUG_PINK: Rgb = Rgb::new(200, 50, 200);

#[cfg(test)]
pub mod test {

    pub fn cap_float(value: f32) -> f32 {
        (value * 10.0).round() / 10.0
    }
}
