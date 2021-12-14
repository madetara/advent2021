use std::collections::HashMap;
use std::fs;

pub fn read_input(filename: &str) -> (String, HashMap<(char, char), char>) {
    let raw_input = fs::read_to_string(filename).unwrap();
    let mut raw_input_iter = raw_input.split("\n\n");

    let polymer = raw_input_iter.next().unwrap().to_string();
    let mut transofrmations = HashMap::new();

    raw_input_iter.next().unwrap().lines().for_each(|s| {
        let mut raw_line = s.split(" -> ");
        let mut from = raw_line.next().unwrap().chars();
        let into = raw_line.next().unwrap().chars().nth(0).unwrap();

        transofrmations.insert((from.next().unwrap(), from.next().unwrap()), into);
    });

    (polymer, transofrmations)
}
