# text-block
![build](https://github.com/AndreyErmilov/text-block/workflows/CI/badge.svg)

*The project is currently under development and is not ready for use.*

### Example
```rust
use text_block::{TextBlock, CharacterStyle, ParagraphStyle, Pt, Mm, Left, TitleCase};

fn main() {
    let face = Face::from_path("../fonts/font.ttf");
    let black = Color::Cmyk::new(0., 0., 0., 1.);
    let character_style = CharacterStyle::build()
        .font(face)
        .size(Pt(12.))
        .tracking(Em(10))
        .leading(Pt(14.4))
        .transform(TitleCase)
        .color(black)
        .finish();
    
    let paragraph_style = ParagraphStyle::build()
        .style(character_style)
        .align(Left)
        .first_line_indent(Pt(10.))
        .space_after(Pt(10.))
        .hyphenate(Hyphenation::default())
        .finish();
    
    let start = Position::new(10., 10.);
    let text_block = TextBlock::build()
        .style(paragraph_style)
        .text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
        .position(start)
        .width(Mm(35.))
        .height(Mm(50.))
        .finish();
    
    assert_eq!(&text_block.next().text(), String::from("Lorem Ipsum"));
    assert_eq!(&text_block.next().position(), Position::new(10., 15.));
}
```
