use std::{collections::HashMap, fs};

pub fn read_input(filename: &str) -> HashMap<String, Vec<String>> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| s.split('-').map(|s| s.to_string()).collect::<Vec<String>>())
        .fold(HashMap::new(), |mut acc, s| {
            acc.entry(s[0].clone()).or_insert(vec![]).push(s[1].clone());
            acc.entry(s[1].clone()).or_insert(vec![]).push(s[0].clone());
            acc
        })
}
