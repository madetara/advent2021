use std::collections::{HashMap, HashSet};

pub fn solve_1(input_data: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    input_data
        .iter()
        .map(|(_, output)| {
            output
                .iter()
                .filter(|s| {
                    let len = s.len();
                    len == 2 || len == 3 || len == 4 || len == 7
                })
                .count()
        })
        .sum::<usize>()
}

pub fn solve_2(input_data: &Vec<(Vec<String>, Vec<String>)>) -> usize {
    let mut result = 0;
    for line in input_data.iter() {
        let mut segment_to_number: HashMap<String, char> = HashMap::new();

        let one = insert_conditionally(&mut segment_to_number, &line.0, '1', |&s| s.len() == 2);

        let four = insert_conditionally(&mut segment_to_number, &line.0, '4', |&s| s.len() == 4);

        let _ = insert_conditionally(&mut segment_to_number, &line.0, '7', |&s| s.len() == 3);

        let _ = insert_conditionally(&mut segment_to_number, &line.0, '8', |&s| s.len() == 7);

        let three = insert_conditionally(&mut segment_to_number, &line.0, '3', |&s| {
            s.len() == 5
                && s.chars()
                    .collect::<HashSet<char>>()
                    .union(&one.chars().collect::<HashSet<char>>())
                    .count()
                    == 5
        });

        let five = insert_conditionally(&mut segment_to_number, &line.0, '5', |&s| {
            s.len() == 5
                && s.chars()
                    .collect::<HashSet<char>>()
                    .union(
                        &four
                            .chars()
                            .collect::<HashSet<char>>()
                            .difference(&one.chars().collect::<HashSet<char>>())
                            .cloned()
                            .collect::<HashSet<char>>(),
                    )
                    .count()
                    == 5
        });

        let _ = insert_conditionally(&mut segment_to_number, &line.0, '2', |&s| {
            s.len() == 5
                && !s
                    .chars()
                    .collect::<HashSet<char>>()
                    .eq(&five.chars().collect())
                && !s
                    .chars()
                    .collect::<HashSet<char>>()
                    .eq(&three.chars().collect())
        });

        let nine = insert_conditionally(&mut segment_to_number, &line.0, '9', |&s| {
            s.len() == 6
                && s.chars()
                    .collect::<HashSet<char>>()
                    .difference(&four.chars().collect::<HashSet<char>>())
                    .count()
                    == 2
        });

        let six = insert_conditionally(&mut segment_to_number, &line.0, '6', |&s| {
            s.len() == 6
                && s.chars()
                    .collect::<HashSet<char>>()
                    .difference(&one.chars().collect::<HashSet<char>>())
                    .count()
                    == 5
        });

        let _ = insert_conditionally(&mut segment_to_number, &line.0, '0', |&s| {
            s.len() == 6
                && !s
                    .chars()
                    .collect::<HashSet<char>>()
                    .eq(&six.chars().collect())
                && !s
                    .chars()
                    .collect::<HashSet<char>>()
                    .eq(&nine.chars().collect())
        });

        result += line
            .1
            .iter()
            .map(|s| {
                let mut key = s.chars().collect::<Vec<char>>();
                key.sort();
                segment_to_number
                    .get(&key.iter().collect::<String>())
                    .unwrap()
            })
            .cloned()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
    }

    result
}

fn insert_conditionally<F: Fn(&&String) -> bool>(
    dict: &mut HashMap<String, char>,
    key_source: &Vec<String>,
    value: char,
    condition: F,
) -> String {
    let mut key = key_source
        .iter()
        .filter(condition)
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();
    key.sort();
    dict.insert(key.iter().cloned().collect::<String>(), value);
    key.iter().collect()
}
