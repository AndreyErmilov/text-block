use crate::styles::CharacterStyle;
use crate::{Color, Face, Size, TextTransformation};

#[derive(Debug, Default)]
pub struct CharacterStyleBuilder {
    pub face: Option<Face>,
    pub size: Option<Size>,
    pub tracking: Option<Size>,
    pub leading: Option<Size>,
    pub transform: Option<TextTransformation>,
    pub color: Option<Color>,
}

impl CharacterStyleBuilder {
    pub fn finish(self) -> CharacterStyle {
        CharacterStyle {
            face: self.face.unwrap_or_default(),
            size: self.size.unwrap_or_default(),
            tracking: self.tracking.unwrap_or(Size::Pt(0.)),
            leading: self.leading.unwrap_or(Size::Pt(0.)),
            transform: self.transform,
            color: self.color.unwrap_or_default(),
        }
    }
    pub fn font<T: Into<Face>>(self, face: T) -> Self {
        Self { face: Some(face.into()), ..self }
    }
    pub fn size(self, size: Size) -> Self {
        Self { size: Some(size), ..self }
    }
    pub fn tracking(self, size: Size) -> Self {
        Self { tracking: Some(size), ..self }
    }
    pub fn leading(self, size: Size) -> Self {
        Self { leading: Some(size), ..self }
    }
    pub fn transform(self, transform: TextTransformation) -> Self {
        Self { transform: Some(transform), ..self }
    }
    pub fn color(self, color: Color) -> Self {
        Self { color: Some(color), ..self }
    }
}
