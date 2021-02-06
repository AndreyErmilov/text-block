use crate::styles::CharacterStyleBuilder;
use crate::{Color, Face, Size, TextTransformation};

#[derive(Debug)]
pub struct CharacterStyle {
    pub face: Face,
    pub size: Size,
    pub tracking: Size,
    pub leading: Size,
    pub transform: Option<TextTransformation>,
    pub color: Color,
}

impl CharacterStyle {
    pub fn build() -> CharacterStyleBuilder {
        CharacterStyleBuilder::default()
    }
}
