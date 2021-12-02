#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub distance: i64,
    pub depth: i64,
    aim: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Submarine {
    pub pos: Position,
}

impl Submarine {
    pub fn new() -> Submarine {
        Self {
            pos: Position {
                distance: 0,
                depth: 0,
                aim: 0,
            },
        }
    }

    pub fn move_(&mut self, dir: Direction, x: i64) {
        let (ddistance, ddepth) = match dir {
            Direction::Forward => (x, 0),
            Direction::Down => (0, x),
            Direction::Up => (0, -x),
        };

        self.pos.depth += ddepth;
        self.pos.distance += ddistance;
    }

    pub fn move_2(&mut self, dir: Direction, x: i64) {
        let (daim, ddistance, ddepth) = match dir {
            Direction::Up => (-x, 0, 0),
            Direction::Down => (x, 0, 0),
            Direction::Forward => (0, x, self.pos.aim * x),
        };

        self.pos.aim += daim;
        self.pos.distance += ddistance;
        self.pos.depth += ddepth;
    }
}
