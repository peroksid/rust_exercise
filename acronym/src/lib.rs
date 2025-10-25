#[derive(Copy, Clone)]
enum State {
    Space,
    Word,
}

pub fn abbreviate(phrase: &str) -> String {
    let mut state = State::Space;
    let mut previous_char_lowercase: Option<bool> = None;
    phrase
        .chars()
        .filter(|&c| match (state, c.is_ascii_alphabetic(), c) {
            (State::Space, true, _) => {
                state = State::Word;
                previous_char_lowercase = Some(c.is_ascii_lowercase());
                true
            }
            (State::Space, false, _) => false,
            (State::Word, true, _) => match (c.is_ascii_lowercase(), previous_char_lowercase) {
                (false, Some(true)) => {
                    previous_char_lowercase = Some(c.is_ascii_lowercase());
                    true
                }
                (_, _) => {
                    previous_char_lowercase = Some(c.is_ascii_lowercase());
                    false
                }
            },
            (State::Word, false, '\'') => {
                previous_char_lowercase = None;
                false
            }
            (State::Word, false, _) => {
                state = State::Space;
                false
            }
        })
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
