use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, Eq)]
struct Point {
    pub weight: usize,
    pub x: usize,
    pub y: usize,
    pub parent: Option<(usize, usize)>,
}

impl Point {
    fn new(weight: usize, x: usize, y: usize, parent: Option<(usize, usize)>) -> Point {
        Point {
            weight,
            x,
            y,
            parent,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return Ordering::Equal;
        }

        if self.weight >= other.weight {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_1(data: &Vec<Vec<u8>>) -> usize {
    solve(data)
}

pub fn solve_2(data: &Vec<Vec<u8>>) -> usize {
    let mut multiplied_data = Vec::new();
    let height = data.len();
    let width = data[0].len();

    for i in 0..(height * 5) {
        multiplied_data.push(Vec::new());
        for j in 0..(width * 5) {
            let mut new_weight = if i / height == 0 && j / width == 0 {
                data[i][j]
            } else if i / height == 0 {
                (multiplied_data[i][j - width] + 1) % 10
            } else {
                (multiplied_data[i - height][j] + 1) % 10
            };

            if new_weight == 0 {
                new_weight += 1;
            }

            multiplied_data[i].push(new_weight);
        }
    }

    solve(&multiplied_data)
}

pub fn solve(data: &Vec<Vec<u8>>) -> usize {
    let mut q: BinaryHeap<Point> = BinaryHeap::new();
    let mut visited: HashMap<(usize, usize), Option<(usize, usize)>> = HashMap::new();
    let starting_point = Point::new(data[0][0] as usize, 0, 0, None);
    let mut finish: Option<Point> = None;
    q.push(starting_point);

    while !q.is_empty() {
        let current = q.pop().unwrap();

        if current.x == data.len() - 1 && current.y == data[0].len() - 1 {
            visited.insert((current.x, current.y), current.parent);
            finish = Some(current);
            break;
        }

        if visited.contains_key(&(current.x, current.y)) {
            continue;
        }

        visited.insert((current.x, current.y), current.parent);

        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|&(dx, dy)| (current.x as isize + dx, current.y as isize + dy))
            .filter(|&(x, y)| {
                x >= 0 && y >= 0 && (x as usize) < data.len() && (y as usize) < data[0].len()
            })
            .map(|(x, y)| (x as usize, y as usize))
            .for_each(|(x, y)| {
                if !visited.contains_key(&(x, y)) {
                    q.push(Point::new(
                        current.weight + (data[x][y] as usize),
                        x,
                        y,
                        Some((current.x, current.y)),
                    ));
                }
            });
    }

    if finish.is_none() {
        panic!("finish not found");
    }

    let mut result = 0;
    let mut current = (finish.unwrap().x, finish.unwrap().y);

    loop {
        let &next = visited.get(&current).unwrap();

        if next.is_none() {
            break;
        }

        result += data[current.0][current.1] as usize;
        current = next.unwrap();
    }

    result
}
