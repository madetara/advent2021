use std::fs;

pub fn read_input(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}
