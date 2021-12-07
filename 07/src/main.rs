use std::fs;

mod io;
mod solver;

fn main() {
    let test_input = io::read_input("test.txt");
    assert!(solver::solve_1(&test_input) == 37);

    let input = io::read_input("in.txt");
    fs::write("1.txt", solver::solve_1(&input).to_string()).unwrap();

    assert!(solver::solve_2(&test_input) == 168);
    fs::write("2.txt", solver::solve_2(&input).to_string()).unwrap();
}
