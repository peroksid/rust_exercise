pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let len = num_str.len();

    num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|d| d.pow(len as u32))
        .sum::<u32>()
        == num
}
