use std::fs;

pub fn read_input(filename: &str) -> [[u8; 10]; 10] {
    let mut result = [[0u8; 10]; 10];

    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, s)| {
            s.chars().enumerate().for_each(|(j, c)| {
                result[i][j] = c.to_digit(10).unwrap() as u8;
            })
        });

    result
}
