#[derive(Debug)]
pub enum Size {
    Pt(f32),
    Mm(f32),
}

impl Default for Size {
    fn default() -> Self {
        Self::Pt(12.)
    }
}
