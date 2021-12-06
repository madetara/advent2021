use std::{collections::HashMap, ops::RangeInclusive};

pub fn solve_1(input: &Vec<((u64, u64), (u64, u64))>) -> u64 {
    let mut map: HashMap<(u64, u64), u64> = HashMap::new();

    fill_map_horizontal_and_vertical(input, &mut map);

    map.iter()
        .filter(|(_, &v)| v >= 2)
        .count()
        .try_into()
        .unwrap()
}

pub fn solve_2(input: &Vec<((u64, u64), (u64, u64))>) -> u64 {
    let mut map: HashMap<(u64, u64), u64> = HashMap::new();

    fill_map_horizontal_and_vertical(input, &mut map);
    fill_map_diagonal(input, &mut map);

    map.iter()
        .filter(|(_, &v)| v >= 2)
        .count()
        .try_into()
        .unwrap()
}

fn fill_map_horizontal_and_vertical(
    input: &Vec<((u64, u64), (u64, u64))>,
    map: &mut HashMap<(u64, u64), u64>,
) {
    for &((a_x, a_y), (b_x, b_y)) in input {
        if a_x != b_x && a_y != b_y {
            continue;
        }

        between_not_smart(a_x, b_x).for_each(|x| {
            between_not_smart(a_y, b_y).for_each(|y| {
                let entry = map.entry((x, y)).or_insert(0);
                *entry += 1;
            })
        })
    }
}

fn fill_map_diagonal(input: &Vec<((u64, u64), (u64, u64))>, map: &mut HashMap<(u64, u64), u64>) {
    for &((a_x, a_y), (b_x, b_y)) in input {
        if a_x == b_x || a_y == b_y {
            continue;
        }

        between_smarter(a_x, b_x)
            .zip(between_smarter(a_y, b_y))
            .for_each(|(x, y)| {
                let entry = map.entry((x, y)).or_insert(0);
                *entry += 1;
            })
    }
}

fn between_not_smart(a: u64, b: u64) -> RangeInclusive<u64> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn between_smarter(a: u64, b: u64) -> Box<dyn Iterator<Item = u64>> {
    if a < b {
        Box::new(a..=b)
    } else {
        Box::new((b..=a).rev())
    }
}
