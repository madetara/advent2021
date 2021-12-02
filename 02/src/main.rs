use crate::core::solver;
use std::fs;

mod core;
mod io;

pub fn main() {
    let input = io::read_input();

    let result1 = solver::solve_part1(&input);
    let result2 = solver::solve_part2(&input);

    fs::write("1.txt", result1.to_string()).unwrap();
    fs::write("2.txt", result2.to_string()).unwrap();
}
