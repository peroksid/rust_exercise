use std::collections::HashMap;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    let brackets = HashMap::from([('(', ')'), ('[', ']'), ('{', '}')]);
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(brackets[&c]),
            ')' | ']' | '}' => {
                if stack.pop().is_none_or(|x| c != x) {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}
