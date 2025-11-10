fn is_flower(garden: &[&str], row_id: usize, col_id: usize) -> bool {
    garden
        .get(row_id)
        .and_then(|row| row.get(col_id..col_id + 1))
        .is_some_and(|cell| cell == "*")
}

fn count_adjacents(garden: &[&str], row_id: usize, col_id: usize) -> u32 {
    (row_id.saturating_sub(1)..=row_id + 1)
        .flat_map(move |r| {
            (col_id.saturating_sub(1)..=col_id + 1).filter(move |c| is_flower(garden, r, *c))
        })
        .count() as u32
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    garden
        .iter()
        .enumerate()
        .map(|(row_id, &row)| {
            row.as_bytes()
                .iter()
                .enumerate()
                .map(|(col_id, &cell)| match cell {
                    b'*' => '*',
                    _ => match count_adjacents(garden, row_id, col_id) {
                        0 => ' ',
                        x => char::from_digit(x, 10).unwrap(),
                    },
                })
                .collect()
        })
        .collect()
}
