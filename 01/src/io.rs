use std::fs;

pub fn read_input() -> Vec<u64> {
    fs::read_to_string("in.txt")
        .unwrap()
        .split('\n')
        .enumerate()
        .filter(|s| !s.1.is_empty())
        .map(|s| s.1.parse::<u64>().unwrap())
        .collect()
}
