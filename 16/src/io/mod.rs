use std::fs;

pub fn read_input(filename: &str) -> String {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .nth(0)
        .unwrap()
        .chars()
        .map(|c| to_binary(c))
        .collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("not hexadecimal"),
    }
}
