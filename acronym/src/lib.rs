enum State {
    Space,
    Word,
}

fn is_space(c: &char) -> bool {
    !c.is_ascii_alphabetic()
}

pub fn abbreviate(phrase: &str) -> String {
    let mut state = State::Space;
    let mut result = String::new();
    for c in phrase.chars() {
        match state {
            State::Space => {
                if !is_space(&c) {
                    state = State::Word;
                    result.push(c.to_ascii_uppercase());
                }
            }
            State::Word => {
                if is_space(&c) {
                    state = State::Space;
                } else if c.is_ascii_uppercase() {
                    result.push(c);
                }
            }
        }
    }
    result
}
