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
    let mut last = ' ';
    for c in phrase.chars() {
        if c == '\'' {
            continue;
        }
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
                } else if last.is_ascii_lowercase() && c.is_ascii_uppercase() {
                    result.push(c);
                }
            }
        }
        last = c;
    }
    result
}
