fn digit_value((i, d): (usize, u32)) -> u32 {
    if i % 2 == 1 {
        let x = d * 2;
        if x > 9 { x - 9 } else { x }
    } else {
        d
    }
}

struct DigitFilterWithMemory {
    invalid_char: bool,
    digit_count: u32,
}

impl DigitFilterWithMemory {
    fn new() -> Self {
        Self {
            invalid_char: false,
            digit_count: 0,
        }
    }
    fn if_digit(&mut self, c: char) -> bool {
        if self.invalid_char {
            return false;
        };
        let is_digit = c.is_ascii_digit();
        if is_digit {
            self.digit_count += 1;
        } else if c != ' ' {
            self.invalid_char = true
        }
        is_digit
    }
}

pub fn is_valid(code: &str) -> bool {
    let mut filter = DigitFilterWithMemory::new();
    let luhn = code
        .chars()
        .rev()
        .filter( |c| filter.if_digit(*c))
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(digit_value)
        .sum::<u32>()
        % 10;
    !filter.invalid_char && (filter.digit_count > 1) && luhn == 0
}
