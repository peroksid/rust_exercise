use std::collections::HashSet;

fn sort_str(s: &str) -> Vec<char> {
    let mut v: Vec<_> = s.chars().collect();
    v.sort_unstable();
    v
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let lowercase = word.to_lowercase();
    let sorted = sort_str(&lowercase);
    possible_anagrams
        .iter()
        .filter(|w| {
            let l = w.to_lowercase();
            l != lowercase && sorted == sort_str(&l)
        })
        .copied()
        .collect()
}
