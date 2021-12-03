use std::fs;

use crate::core::solver;

mod core;
mod io;

fn main() {
    let test_input = io::read_input_1("test.txt");
    assert!(core::solver::solve_1(test_input) == 198);

    let input = io::read_input_1("in.txt");
    fs::write("1.txt", solver::solve_1(input).to_string()).unwrap();

    let test_input2 = io::read_input_2("test.txt");
    assert!(core::solver::solve_2(test_input2) == 230);

    let input2 = io::read_input_2("in.txt");
    fs::write("2.txt", solver::solve_2(input2).to_string()).unwrap();
}
