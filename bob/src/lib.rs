pub fn reply(message: &str) -> &str {
    let trimmed_message = message.trim();
    let any_lowercase = trimmed_message.chars().any(char::is_lowercase);
    let any_uppercase = trimmed_message.chars().any(char::is_uppercase);
    let is_yelling = any_uppercase && !any_lowercase;
    let is_question = trimmed_message.ends_with('?');
    match (is_yelling, is_question) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Whoa, chill out!",
        (false, true) => "Sure.",
        (false, false) => {
            if trimmed_message.is_empty() {
                "Fine. Be that way!"
            } else {
                "Whatever."
            }
        }
    }
}
