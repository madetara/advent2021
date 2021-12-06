pub struct GameState<'a> {
    fields: &'a Vec<Vec<u64>>,
    completed: Vec<Vec<bool>>,
    finished: Vec<bool>,
}

impl<'a> GameState<'a> {
    pub fn new(fields: &Vec<Vec<u64>>) -> GameState {
        let completed = (0..fields.len())
            .map(|_| (0..25).map(|_| false).collect())
            .collect();
        let finished = (0..fields.len()).map(|_| false).collect();
        GameState {
            fields,
            completed,
            finished,
        }
    }

    pub fn make_move(self: &mut GameState<'a>, value: u64) -> Option<u64> {
        let mut winner: Option<u64> = None;

        for (i, field) in self.fields.iter().enumerate() {
            match field.iter().enumerate().find_map(|(j, x)| {
                if x.eq(&value) {
                    return Some((j, x));
                }
                None
            }) {
                Some((j, x)) => {
                    self.completed[i][j] = true;

                    match self.get_winner_score(i, j, *x) {
                        Some(x) => winner = Some(x),
                        None => {}
                    }
                }
                None => {}
            }
        }

        winner
    }

    fn get_winner_score(self: &mut GameState<'a>, i: usize, j: usize, x: u64) -> Option<u64> {
        if !self.is_winner(i, j) {
            return None;
        }

        self.finished[i] = true;
        Some(
            self.fields[i]
                .iter()
                .enumerate()
                .filter_map(|(j, &x)| {
                    if !self.completed[i][j] {
                        return Some(x);
                    }
                    None
                })
                .sum::<u64>()
                * x,
        )
    }

    fn is_winner(self: &GameState<'a>, i: usize, j: usize) -> bool {
        let row = j / 5;
        let column = j % 5;

        !self.finished[i]
            && ((0..5).all(|d| self.completed[i][5 * row + d])
                || (0..5).all(|d| self.completed[i][5 * d + column]))
    }
}
