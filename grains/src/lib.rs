pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1 << (s - 1),
        _ => panic!("invalid {s}"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
