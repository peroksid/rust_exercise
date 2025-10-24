use std::cell::Cell;
use std::iter::{once_with, repeat_n};

#[inline(always)]
fn power(base: u64, exponent: u32) -> u64 {
    base.pow(exponent)
}

#[inline(always)]
fn divisible_by(number: u64, base: u64, exponent: u32) -> bool {
    number % power(base, exponent) == 0
}

fn count_of_factor(number: u64, factor: u64) -> usize {
    (1..)
        .take_while(move |exponent| divisible_by(number, factor, *exponent))
        .count()
}

fn handle_factor(n: &Cell<u64>, base: u64) -> impl Iterator<Item = u64> {
    let count = count_of_factor(n.get(), base);
    n.update(|n| n / power(base, count as u32));
    repeat_n(base, count)
}

pub fn factors(n: u64) -> Vec<u64> {
    let n = Cell::new(n);
    handle_factor(&n, 2)
        .chain(
            (3..)
                .step_by(2)
                .take_while(|x| x * x <= n.get())
                .flat_map(|x| handle_factor(&n, x)),
        )
        .chain(once_with(|| n.get()).take_while(|_| n.get() > 1))
        .collect()
}
