use std::collections::HashMap;

const DIGITS: [char;10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']; 
const DIGITS_FOR_FIRST: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
type DigitList<'static> = &'static [char];

type LetterVariants = HashMap<char, DigitList>;

type Variant = HashMap<char, char>;

type VariantList = Vec<Variant>;


struct Variants {
    letter_variants: LetterVariants,
    indexes: HashMap<char, u8>,
    order: Vec<char>,
}


impl Variants {
    fn new(letter_variants: LetterVariants) -> Self {
        let mut indexes = HashMap::new();
        for k in letter_variants.keys() {
            indexes.insert(*k, 0);
        }
        Self {
            letter_variants: letter_variants.clone(),
            indexes: indexes,
            order: letter_variants.keys().map(|&x| x).collect::<Vec<char>>()
        }
    }
}

impl Iterator for Variants {
    type Item = Variant;
    fn next(&mut self) -> Option<Self::Item> {
        /*let mut m = HashMap::new();
        let mut progressed = false;
        let mut need_tick = 
        for k in self.order.iter() {
            if self.indexes[k] != self.letter_variants[k].len() - 1 {
                self.indexes[k] += 1;
                progressed = true;
                break;
            }

        }
        */
        None
    }

}

fn drop_char(c: char, digits: &[char]) -> Vec<char> {
    let mut r = digits.to_owned();
    r.retain(|&x| x != c);
    r
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

fn digify_dict(d: Variant) -> HashMap<char, u8> {
    d.iter()
        .map(|(&k, &v)| (k, v.to_digit(10).unwrap() as u8))
        .collect::<_>()
}

fn collect_letters_variants(input: &str) -> LetterVariants {
    let mut m = HashMap::new();
    let mut next_is_first = true;
    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            if next_is_first {
                m.insert(c, &DIGITS_FOR_FIRST);
            } else {
                if !m.contains_key(&c) {
                    m.insert(c, &DIGITS);
                }
            }
            next_is_first = false;
        } else {
            next_is_first = true;
        }
    }
    m
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
   let (left_expr_list, right_expr_list) = get_exprs_from_input(input);
   let letter_variants = collect_letters_variants(input);
   let variants = Variants::new(letter_variants);
    for variant in variants {
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
