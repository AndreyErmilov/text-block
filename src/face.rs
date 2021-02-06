#[derive(Debug)]
pub struct Face(String);

impl<T> From<T> for Face
where
    T: ToString,
{
    fn from(source: T) -> Self {
        Self(source.to_string())
    }
}

impl Default for Face {
    fn default() -> Self {
        Self(String::from("../fonts/arial.ttf"))
    }
}
