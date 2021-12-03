use std::collections::{HashMap, HashSet};

pub fn solve_1((input, row_count): (HashMap<usize, usize>, usize)) -> isize {
    let len = input.len();

    let mut gamma = String::new();
    let mut epsilon = String::new();

    for i in 0..len {
        if *input.get(&i).unwrap() > row_count / 2 {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

pub fn solve_2(raw_input: Vec<String>) -> isize {
    let gamma_i = solve_2_internal(&raw_input, '0', '1');
    let epsilon_i = solve_2_internal(&raw_input, '1', '0');

    let gamma = &raw_input[gamma_i];
    let epsilon = &raw_input[epsilon_i];

    isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap()
}

fn solve_2_internal(raw_input: &Vec<String>, a: char, b: char) -> usize {
    let len = raw_input.len();
    let bytes_len = raw_input.first().unwrap().len();

    let mut possible_answers = Box::new(HashSet::new());
    (0..len).for_each(|x| {
        possible_answers.insert(x);
    });

    for j in 0..bytes_len {
        if possible_answers.len() == 1 {
            break;
        }

        let mut possible_answers_next: HashSet<usize> = HashSet::new();

        let frequencies = get_frequencies(&raw_input, &possible_answers);
        let to_keep = if frequencies[&j] < 0 { a } else { b };

        for i in possible_answers.as_ref() {
            let c = raw_input[*i].chars().nth(j).unwrap();

            if c.eq(&to_keep) {
                possible_answers_next.insert(*i);
            }
        }

        possible_answers = Box::new(possible_answers_next);
    }

    *possible_answers.as_ref().into_iter().nth(0).unwrap()
}

fn get_frequencies(raw_input: &Vec<String>, indices: &Box<HashSet<usize>>) -> HashMap<usize, i32> {
    let mut result = HashMap::new();

    for i in indices.as_ref() {
        raw_input[*i].chars().enumerate().for_each(|(i, c)| {
            let entry = result.entry(i).or_insert(0);
            *entry += if c.eq(&'0') { -1 } else { 1 };
        })
    }

    result
}
