pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|d| get_one_strophe_for_number_of_bottles(start_bottles.saturating_sub(d)))
        .reduce(|a, b| a + "\n\n" + &b)
        .unwrap()
}

fn get_one_strophe_for_number_of_bottles(n: u32) -> String {
    let old = word(n);
    let old_capital = capitalize(old);
    let old_plural = plural_for(n);
    let new_n = n.saturating_sub(1);
    let new = word(new_n);
    let new_plural = plural_for(new_n);
    format!(
        "{old_capital} green bottle{old_plural} hanging on the wall,\n\
        {old_capital} green bottle{old_plural} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {new} green bottle{new_plural} hanging on the wall."
    )
}

fn word(n: u32) -> &'static str {
    match n {
        1 => "one", 
        2 => "two", 
        3 => "three", 
        4 => "four", 
        5 => "five", 
        6 => "six", 
        7 => "seven", 
        8 => "eight", 
        9 => "nine", 
        10 => "ten",
        _ => "no",
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn plural_for(n: u32) -> &'static str {
    if n == 1 {
        ""
    } else {
        "s"
    }
}
