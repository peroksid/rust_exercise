pub fn egg_count(display_value: u32) -> usize {
    (0..u32::BITS)
        .filter(|n| display_value >> n & 1 == 1)
        .count()
}
