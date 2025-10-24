const POTS_PER_STUDENT: usize = 2;

fn get_student_column(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("unknown student name"),
    }
}

fn get_plant_name(code: char) -> &'static str {
    match code {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => panic!("unknown plant code"),
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let column = get_student_column(student);
    diagram
        .lines()
        .flat_map(|line| {
            line.chars()
                .skip(column * POTS_PER_STUDENT)
                .take(POTS_PER_STUDENT)
        })
        .map(get_plant_name)
        .collect()
}
