pub fn nth(n: u32) -> u32 {
    (2u32..)
        .filter(|candidate|(2..=candidate.isqrt()).all(|d| candidate % d != 0 ) )
        .nth(n as usize)
        .unwrap()
}
