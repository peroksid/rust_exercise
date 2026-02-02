fn digit_value((i, d): (usize, u32)) -> u32 {
    if i % 2 == 1 {
        let x = d * 2;
        if x > 9 { x - 9 } else { x }
    } else {
        d
    }
}

pub fn is_valid(code: &str) -> bool {
    let mut length = 0;
    let mut sum = 0;
    let mut is_valid = true;
    let check_if_valid = |x: &char| {
        is_valid = x.is_ascii_digit();
        is_valid
    };
    for d in code
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .rev()
        .take_while(check_if_valid)
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(digit_value)
    {
        sum += d;
        length += 1;
    }
    is_valid && length > 1 && sum % 10 == 0
}
