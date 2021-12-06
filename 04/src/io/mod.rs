use std::fs;

pub fn read_input(filename: &str) -> (Vec<u64>, Vec<Vec<u64>>) {
    let raw_blocks: Vec<String> = fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();

    let moves: Vec<u64> = raw_blocks
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let bingos: Vec<Vec<u64>> = raw_blocks
        .iter()
        .skip(1)
        .map(|s| {
            s.lines()
                .flat_map(|s| s.split_whitespace())
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    (moves, bingos)
}
