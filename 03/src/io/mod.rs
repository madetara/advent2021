use std::{collections::HashMap, fs};

pub fn read_input_1(filename: &str) -> (HashMap<usize, usize>, usize) {
    let mut ones_per_column = HashMap::new();
    let mut row_count = 0;

    fs::read_to_string(filename).unwrap().lines().for_each(|s| {
        row_count += 1;
        s.chars().enumerate().for_each(|(i, c)| {
            if c.eq(&'1') {
                let entry = ones_per_column.entry(i).or_insert(0);
                *entry += 1;
            }
        });
    });

    (ones_per_column, row_count)
}

pub fn read_input_2(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect()
}
