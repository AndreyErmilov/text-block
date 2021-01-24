# text-block
*The project is currently under development and is not ready for use in production.*

### Example
```rust
use text_block::{TextBlockBuilder, Pt, Mm, Left, TitleCase};

fn main() {
    let face = Face::from_path("../fonts/font.ttf");
    let black = Color::Cmyk::new(0., 0., 0., 1.);
    let start = Position::new(10., 10.);
    let text_block = TextBlockBuilder::new()
        .font(face)
        .text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
        .size(Pt(12.))
        .position(start)
        .width(Mm(35.))
        .hyphenate()
        .transform(TitleCase)
        .spacing(Pt(1.2))
        .color(black)
        .align(Left)
        .finish();
    assert_eq!(&text_block.next().text(), String::from("Lorem ipsum"));
    assert_eq!(&text_block.next().position(), Position::new(10., 15.));
}
```
