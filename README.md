# text-block
*The project is currently under development and is not ready for use in production.*

### Example
```rust
use text_block::{TextBlock, Pt, Mm, Align};

let text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
let text_block = TextBlock::build()
    .text(text)
    .face("../fonts/font.ttf")
    .size(Pt(12))
    .width(Mm(35.))
    .hyphenate()
    .align(Align::Left);

assert_eq!(&text_block.next(), String::from("Lorem ipsum"));
assert_eq!(&text_block.next(), String::from("dolor sit amet,")); 
assert_eq!(&text_block.next(), String::from("consectetur"));
assert_eq!(&text_block.next(), String::from("adipiscing elit, sed")); 
assert_eq!(&text_block.next(), String::from("do eiusmod"));
assert_eq!(&text_block.next(), String::from("tempor incididunt"));
assert_eq!(&text_block.next(), String::from("ut labore et dolore"));
assert_eq!(&text_block.next(), String::from("magna aliqua."));
```
