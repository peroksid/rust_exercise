pub fn build_proverb(list: &[&str]) -> String {
    if let Some(subject) = list.first() {
        list
            .windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain([format!("And all for the want of a {}.", subject)])
            .collect()
    } else {
        String::new()
    }
}
