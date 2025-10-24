fn is_flower(garden: &[&str], row_id: usize, col_id: usize) -> bool {
    garden[row_id].as_bytes()[col_id] == b'*'
}

fn count_adjacents(garden: &[&str], row_id: usize, col_id: usize) -> u32 {
    let existing_rows = 0..garden.len();
    let existing_cols = 0..garden[0].len();
    (row_id.saturating_sub(1)..=row_id + 1)
        .map(|r| {
            (col_id.saturating_sub(1)..=col_id + 1)
                .filter(|&c| {
                    !(r == row_id && c == col_id)
                        && existing_rows.contains(&r)
                        && existing_cols.contains(&c)
                })
                .filter(|c| is_flower(garden, r, *c))
                .count() as u32
        })
        .sum()
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }
    if garden[0].is_empty() {
        return vec![String::new()];
    }
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
