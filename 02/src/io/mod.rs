use crate::core::submarine::{self, Direction};
use std::fs;

pub fn read_input() -> Vec<(submarine::Direction, i64)> {
    fs::read_to_string("in.txt")
        .unwrap()
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let raw_data: Vec<&str> = s.split(' ').collect();

            (
                match raw_data[0] {
                    "forward" => Direction::Forward,
                    "up" => Direction::Up,
                    "down" => Direction::Down,
                    _ => panic!("wrong input"),
                },
                raw_data[1].parse::<i64>().unwrap(),
            )
        })
        .collect()
}
