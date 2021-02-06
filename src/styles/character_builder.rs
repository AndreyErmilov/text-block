use crate::styles::CharacterStyle;
use crate::{Color, Face, Size, TextTransformation};

/// A character style builder
///
/// This type can be used to construct an instance of `CharacterStyle`
/// through a builder-like pattern.
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
    /// Generate character style with current settings.
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

    /// Set path to font.
    pub fn font<T: Into<Face>>(self, face: T) -> Self {
        Self { face: Some(face.into()), ..self }
    }

    /// Set font size.
    pub fn size(self, size: Size) -> Self {
        Self { size: Some(size), ..self }
    }

    /// Set font tracking value.
    pub fn tracking(self, size: Size) -> Self {
        Self { tracking: Some(size), ..self }
    }

    /// Set font leading (distance between text lines) value.
    pub fn leading(self, size: Size) -> Self {
        Self { leading: Some(size), ..self }
    }

    /// Set text transformation like UPPER CASE ot Title Case.
    pub fn transform(self, transform: TextTransformation) -> Self {
        Self { transform: Some(transform), ..self }
    }

    /// Set text color.
    pub fn color(self, color: Color) -> Self {
        Self { color: Some(color), ..self }
    }
}
