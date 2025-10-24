use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&cost, chars)| chars.iter().map(move |&ch| (ch.to_ascii_lowercase(), cost)))
        .collect()
}
