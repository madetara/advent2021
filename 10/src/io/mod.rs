use std::fs;

pub fn read_input(filename: &str) -> Vec<Vec<char>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect()
}
