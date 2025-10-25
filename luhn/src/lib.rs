pub fn is_valid(code: &str) -> bool {
    let mut invalid_char = false;
    let mut digit_count = 0;
    let luhn = code
        .chars()
        .rev()
        .filter(|c| {
            let is_digit = c.is_ascii_digit();
            if !(is_digit || *c == ' ') {
                invalid_char = true
            };
            if is_digit {
                digit_count += 1;
            }
            is_digit
        })
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 1 {
                let x = d * 2;
                if x > 9 { x - 9 } else { x }
            } else {
                d
            }
        })
        .sum::<u32>()
        % 10;
    (digit_count > 1) && !invalid_char && luhn == 0
}
