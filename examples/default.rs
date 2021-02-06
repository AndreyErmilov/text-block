use text_block::CharacterStyle;

fn main() {
    let character = CharacterStyle::build()
        .font("../fonts/test.ttf")
        .finish();
    dbg!(&character);
}
