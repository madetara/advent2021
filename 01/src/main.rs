use std::fs;
mod io;
mod solver;

fn main() {
    let input = io::read_input();

    let part1 = solver::solve_part1(&input);
    fs::write("part1.txt", part1.to_string()).unwrap();

    let part2 = solver::solve_part2(&input);
    fs::write("part2.txt", part2.to_string()).unwrap();
}
