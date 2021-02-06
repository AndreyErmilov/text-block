use crate::styles::CharacterStyleBuilder;
use crate::{Color, Face, Size, TextTransformation};

/// `CharacterStyle` structure describes a set of base formatting attributes
/// which can be applied to text in one step.
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
    /// Start building `CharacterStyle`
    pub fn build() -> CharacterStyleBuilder {
        CharacterStyleBuilder::default()
    }
}
