#[derive(Debug)]
pub enum Color {
    Cmyk(f32, f32, f32, f32),
    Rgb(u8, u8, u8),
    Pantone(u16),
}

impl Default for Color {
    fn default() -> Self {
        Self::Cmyk(0., 0., 0., 1.)
    }
}
