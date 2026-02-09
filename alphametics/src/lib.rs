use std::collections::HashMap;
use std::collections::HashSet;

fn drop_char(c: char, digits: &[char]) -> Vec<char> {
    let mut r = digits.to_owned();
    r.retain(|&x| x != c);
    r
}

type Variant = HashMap<char, char>;
type VariantList = Vec<Variant>;

fn gen_rec_single(chars: &mut Vec<char>, digits: Vec<char>) -> VariantList {
    let c = chars.pop().unwrap();
    let mut result = Vec::new();
    for d in digits {
        let mut m = HashMap::new();
        m.insert(c, d);
        result.push(m);
    }
    result
}

fn gen_rec_multi(chars: &mut Vec<char>, digits: Vec<char>) -> VariantList {
    let c = chars.pop().unwrap();
    let mut result = Vec::new();
    for d in &digits.clone() {
        let mut m: Variant = HashMap::new();
        m.insert(c, *d);
        let mut next_chars = chars.to_owned();
        for inside_dict in gen_rec(&mut next_chars, drop_char(*d, &digits)) {
            for (k, v) in inside_dict {
                m.insert(k, v);
            }
            result.push(m.to_owned());
        }
    }
    chars.push(c);
    result
}

fn gen_rec(chars: &mut Vec<char>, digits: Vec<char>) -> VariantList {
    match chars.len() {
        0 => Vec::new(),
        1 => gen_rec_single(chars, digits),
        _ => gen_rec_multi(chars, digits),
    }
}

fn extract_expression(expr: String) -> Vec<String> {
    expr.split('+').map(|x| x.trim().to_string()).collect()
}

fn get_exprs_from_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut it = input.split("==");
    (
        extract_expression(it.next().unwrap().to_string()),
        extract_expression(it.next().unwrap().to_string()),
    )
}

fn translate_term(dict: &Variant, term: &str) -> u64 {
    term.chars()
        .map(|x| dict.get(&x).unwrap())
        .collect::<String>()
        .parse::<_>()
        .unwrap()
}

fn calculate_expr_sum(dict: &Variant, expr_list: &[String]) -> Result<u64, String> {
    for x in expr_list {
        if *dict.get(&x.chars().next().unwrap()).unwrap() == '0' {
            return Err("Leading zero".to_string());
        }
    }
    Ok(expr_list.iter().map(|x| translate_term(dict, x)).sum())
}

fn collect_unique_letters(input: &str) -> Vec<char> {
    input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<_>()
}

fn digify_dict(d: Variant) -> HashMap<char, u8> {
    d.iter()
        .map(|(&k, &v)| (k, v.to_digit(10).unwrap() as u8))
        .collect::<_>()
}

fn list_variants(input: &str) -> VariantList {
    gen_rec(
        &mut collect_unique_letters(input),
        vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'],
    )
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (left_expr_list, right_expr_list) = get_exprs_from_input(input);
    for variant in list_variants(input) {
        match calculate_expr_sum(&variant, &left_expr_list) {
            Ok(left) => match calculate_expr_sum(&variant, &right_expr_list) {
                Ok(right) => {
                    if left == right {
                        return Some(digify_dict(variant));
                    }
                }
                Err(_) => (),
            },
            Err(_) => (),
        }
    }
    None
}
