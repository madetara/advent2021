use std::fs;

pub fn read_input(filename: &str) -> Vec<i64> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}
