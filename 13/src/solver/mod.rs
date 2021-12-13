use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
pub enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

pub fn solve_1((points, folds): &(HashSet<(usize, usize)>, Vec<Fold>)) -> usize {
    let &first_fold = folds.first().unwrap();
    fold(Box::new(points.clone()), first_fold).len()
}

pub fn solve_2((points, folds): &(HashSet<(usize, usize)>, Vec<Fold>)) {
    let result = folds
        .iter()
        .fold(Box::new(points.clone()), |p, f| Box::new(fold(p, *f)));

    let height = result.as_ref().iter().max_by_key(|(_, y)| y).unwrap().1;
    let width = result.as_ref().iter().max_by_key(|(x, _)| x).unwrap().0;

    for j in 0..=height {
        for i in 0..=width {
            if result.as_ref().contains(&(i, j)) {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fold(points: Box<HashSet<(usize, usize)>>, fold: Fold) -> HashSet<(usize, usize)> {
    let mut new_points = HashSet::new();

    for &(x, y) in points.as_ref().iter() {
        if match fold {
            Fold::Horizontal(h) => y == h,
            Fold::Vertical(h) => x == h,
        } {
            continue;
        }

        if match fold {
            Fold::Horizontal(h) => y > h,
            Fold::Vertical(h) => x > h,
        } {
            let folded_point = match fold {
                Fold::Horizontal(h) => (x, 2 * h - y),
                Fold::Vertical(h) => (2 * h - x, y),
            };
            new_points.insert(folded_point);
        } else {
            new_points.insert((x, y));
        }
    }

    new_points
}
