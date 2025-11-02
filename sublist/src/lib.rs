use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn find_short_in_long(short: &[i32], long: &[i32]) -> bool {
    short.is_empty() || long.windows(short.len()).any(|x| short.eq(x))
}

pub fn sublist(left: &[i32], right: &[i32]) -> Comparison {
    match left.len().cmp(&right.len()) {
        Ordering::Equal if left == right => Comparison::Equal,
        Ordering::Greater if find_short_in_long(right, left) => Comparison::Superlist,
        Ordering::Less if find_short_in_long(left, right) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
