use std::collections::LinkedList;

pub fn solve_1(data: &Vec<Vec<char>>) -> usize {
    let mut result = 0;

    for line in data.iter() {
        let mut opening_brackets = LinkedList::new();

        for &c in line.iter() {
            if is_opening_bracket(c) {
                opening_brackets.push_front(c);
                continue;
            }

            if !opening_brackets.front().unwrap().eq(&matching_bracket(c)) {
                result += get_score(c);
                break;
            }

            opening_brackets.pop_front().unwrap();
        }
    }

    result
}

pub fn solve_2(data: &Vec<Vec<char>>) -> usize {
    let mut result = vec![];

    for line in data.iter() {
        let mut opening_brackets = LinkedList::new();
        let mut is_corrupted = false;

        for &c in line.iter() {
            if is_opening_bracket(c) {
                opening_brackets.push_front(c);
                continue;
            }

            if !opening_brackets.front().unwrap().eq(&matching_bracket(c)) {
                is_corrupted = true;
                break;
            }

            opening_brackets.pop_front().unwrap();
        }

        if is_corrupted {
            continue;
        }

        let mut score = 0;

        while !opening_brackets.is_empty() {
            let next_bracket = opening_brackets.pop_front().unwrap();

            score = score * 5 + get_score(next_bracket);
        }

        result.push(score);
    }

    result.sort();
    result[result.len() / 2]
}

fn is_opening_bracket(bracket: char) -> bool {
    bracket == '[' || bracket == '(' || bracket == '{' || bracket == '<'
}

fn matching_bracket(bracket: char) -> char {
    match bracket {
        ']' => '[',
        '}' => '{',
        ')' => '(',
        '>' => '<',
        '[' => ']',
        '{' => '}',
        '(' => ')',
        '<' => '>',
        _ => panic!("wtf"),
    }
}

fn get_score(bracket: char) -> usize {
    match bracket {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("not a bracket"),
    }
}
