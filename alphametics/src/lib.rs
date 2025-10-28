use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut tokens: Vec<&str> = input.split(|c: char| !c.is_ascii_alphabetic()).collect::<Vec<&str>>().into_iter().filter(|&s| !s.is_empty()).collect();
    if tokens.len() < 3 {
        return None;
    }
    let result_token = tokens.swap_remove(tokens.len()-1);
    if tokens.iter().map(|x| x.len()).max().unwrap() > result_token.len() {
        return None;
    }
    println!("{:?} {:?}", tokens, result_token);
    todo!("meh");
}
