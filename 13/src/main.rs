mod io;
mod solver;

use std::fs;

fn main() {
    let test_input = io::read_input("test.txt");
    assert!(solver::solve_1(&test_input) == 17);

    let input = io::read_input("in.txt");
    fs::write("1.txt", solver::solve_1(&input).to_string()).unwrap();

    solver::solve_2(&input);
}
