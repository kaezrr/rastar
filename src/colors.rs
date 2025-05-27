pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub const fn new(hexcode: u32) -> Self {
        Color {
            r: (hexcode >> 16) as u8,
            g: (hexcode >> 8) as u8,
            b: hexcode as u8,
        }
    }

    pub fn as_u32(&self) -> u32 {
        u32::from(self.r) << 16 | u32::from(self.g) << 8 | u32::from(self.b)
    }

    pub fn scaled(&self, factor: f64) -> Self {
        Color {
            r: (self.r as f64 * factor).round().clamp(0.0, 255.0) as u8,
            g: (self.g as f64 * factor).round().clamp(0.0, 255.0) as u8,
            b: (self.b as f64 * factor).round().clamp(0.0, 255.0) as u8,
        }
    }
}

#[allow(dead_code)]
pub mod constants {
    use super::Color;
    pub const RED: Color = Color::new(0xff0000);
    pub const GREEN: Color = Color::new(0x00ff00);
    pub const BLUE: Color = Color::new(0x0000ff);
    pub const WHITE: Color = Color::new(0xffffff);
}
