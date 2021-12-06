use std::fs;

pub fn read_input(filename: &str) -> Vec<u8> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .take(1)
        .flat_map(|s| s.split(',').map(|c| c.parse::<u8>().unwrap()))
        .collect()
}
