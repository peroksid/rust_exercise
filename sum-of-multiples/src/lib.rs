pub fn sum_of_multiples(level: u32, factors: &[u32]) -> u32 {
    (1..level)
        .filter(|game_level| {
            factors
                .iter()
                .any(|base_value| *base_value != 0 && game_level % base_value == 0)
        })
        .sum()
}
