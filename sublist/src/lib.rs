use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn find_short_in_long(short: &[i32], long: &[i32], result_when_found: Comparison) -> Comparison {
    if long.windows(short.len()).any(|x| short.eq(x)) {
        result_when_found
    } else {
        Comparison::Unequal
    }
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    match first_list {
        [] => match second_list {
            [] => Comparison::Equal,
            _ => Comparison::Sublist,
        },
        _ => match second_list {
            [] => Comparison::Superlist,
            _ => match first_list.len().cmp(&second_list.len()) {
                Equal => match first_list.iter().eq(second_list.iter()) {
                    true => Comparison::Equal,
                    false => Comparison::Unequal,
                },
                Less => find_short_in_long(first_list, second_list, Comparison::Sublist),
                Greater => find_short_in_long(second_list, first_list, Comparison::Superlist),
            },
        },
    }
}
