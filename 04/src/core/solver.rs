use super::game;

pub fn solve_1(input: &(Vec<u64>, Vec<Vec<u64>>)) -> u64 {
    let mut state = game::GameState::new(&input.1);

    for &value in &input.0 {
        match state.make_move(value) {
            Some(x) => return x,
            None => continue,
        }
    }

    panic!("winner not found");
}

pub fn solve_2(input: &(Vec<u64>, Vec<Vec<u64>>)) -> u64 {
    let mut state = game::GameState::new(&input.1);
    let mut last_winner = None;

    for &value in &input.0 {
        match state.make_move(value) {
            Some(x) => {
                last_winner = Some(x);
            }
            None => continue,
        }
    }

    match last_winner {
        Some(x) => return x,
        None => panic!("winner not found"),
    }
}
