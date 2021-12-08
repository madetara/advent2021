use std::fs;

pub fn read_input(filename: &str) -> Vec<(Vec<String>, Vec<String>)> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            let mut s = s.split(" | ");
            (
                s.next()
                    .unwrap()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect(),
                s.next()
                    .unwrap()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect(),
            )
        })
        .collect()
}
