use std::{
    collections::{HashMap, VecDeque},
    mem::swap,
};

pub fn solve_1(input: &(String, HashMap<(char, char), char>)) -> usize {
    solve_dumb(input, 10)
}

pub fn solve_2(input: &(String, HashMap<(char, char), char>)) -> usize {
    solve_smart(input, 40)
}

fn solve_smart(input: &(String, HashMap<(char, char), char>), steps: usize) -> usize {
    let polymer = input.0.chars().collect::<Vec<char>>();
    let mut polymer_pairs = HashMap::new();
    let mut frequency_map = HashMap::new();

    for i in 0..(polymer.len() - 1) {
        *polymer_pairs
            .entry((polymer[i], polymer[i + 1]))
            .or_insert(0) += 1;

        *frequency_map.entry(polymer[i]).or_insert(0) += 1;
    }

    *frequency_map.entry(polymer[polymer.len() - 1]).or_insert(0) += 1;

    for _ in 0..steps {
        let mut polymer_pairs_new: HashMap<(char, char), usize> = HashMap::new();

        for (&(l, r), &v) in polymer_pairs.iter() {
            match input.1.get(&(l, r)) {
                Some(&c) => {
                    *polymer_pairs_new.entry((l, c)).or_insert(0) += v;
                    *polymer_pairs_new.entry((c, r)).or_insert(0) += v;
                    *frequency_map.entry(c).or_insert(0) += v;
                }
                None => *polymer_pairs_new.entry((l, r)).or_insert(0) += v,
            };
        }

        swap(&mut polymer_pairs, &mut polymer_pairs_new);
    }

    let max = frequency_map.iter().max_by_key(|&(_, v)| v).unwrap();
    let min = frequency_map.iter().min_by_key(|&(_, v)| v).unwrap();

    max.1 - min.1
}

fn solve_dumb(input: &(String, HashMap<(char, char), char>), steps: usize) -> usize {
    let polymer = input.0.chars().collect();

    let resulting_polymer = (0..steps).fold(polymer, |p, _| apply_transformations(&p, &input.1));

    let mut quantities = HashMap::new();

    for c in resulting_polymer {
        *quantities.entry(c).or_insert(0) += 1;
    }

    let max = quantities.iter().max_by_key(|(_, &v)| v).unwrap();
    let min = quantities.iter().min_by_key(|(_, &v)| v).unwrap();

    max.1 - min.1
}

fn apply_transformations(
    polymer: &VecDeque<char>,
    transformations: &HashMap<(char, char), char>,
) -> VecDeque<char> {
    let mut result = VecDeque::new();

    for i in 0..(polymer.len() - 1) {
        result.push_back(polymer[i]);

        match transformations.get(&(polymer[i], polymer[i + 1])) {
            Some(&c) => result.push_back(c),
            None => continue,
        }
    }

    result.push_back(polymer[polymer.len() - 1]);

    result
}
