pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.chars().count() < len {
        return vec![];
    }
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(|window_slice| window_slice.iter().collect())
        .collect()
}
