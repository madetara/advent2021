use std::fs;
mod io;
mod solver;

fn main() {
    let input = io::read_input();

    let part1 = solver::solve_part1(&input);
    let part2 = solver::solve_part2(&input);

    fs::write("1.txt", part1.to_string()).unwrap();
    fs::write("2.txt", part2.to_string()).unwrap();
}
