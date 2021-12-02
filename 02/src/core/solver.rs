use super::submarine;

pub fn solve_part1(input: &[(submarine::Direction, i64)]) -> i64 {
    let mut sub = submarine::Submarine::new();

    for (dir, distance) in input {
        sub.move_(*dir, *distance);
    }

    sub.pos.depth * sub.pos.distance
}

pub fn solve_part2(input: &[(submarine::Direction, i64)]) -> i64 {
    let mut sub = submarine::Submarine::new();

    for (dir, distance) in input {
        sub.move_2(*dir, *distance);
    }

    sub.pos.depth * sub.pos.distance
}
