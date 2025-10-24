/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() < 2 {
        return false;
    }
    code
        .chars()
        .rev()
        .map(|c| {
            c.to_digit(10).unwrap()
        })
        .enumerate()
        .map(|(i, d)| {
            if i % 2 == 1 {
                let x = d * 2;
                if x > 9 {
                    x - 9
                } else {
                    x
                }
            } else {
                d
            }
        })
        .sum::<u32>() % 10 == 0
}
