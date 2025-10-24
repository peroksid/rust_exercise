pub fn square_of_sum(n: u32) -> u32 {
    let sum = (1 + n) * n / 2;
    sum.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1..=n).map(|x| x.pow(2)).sum::<u32>()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
