use std::{collections::HashMap, mem::swap};

pub fn solve_1(input: &Vec<u8>) -> u64 {
    solve_dumb(input, 80)
}

pub fn solve_2(input: &Vec<u8>) -> u64 {
    solve_smart(input, 256)
}

fn solve_dumb(input: &Vec<u8>, day_count: u64) -> u64 {
    let mut data = input.clone();

    (0..day_count).for_each(|_| {
        let current_len = data.len();

        (0..current_len).for_each(|i| {
            if data[i] == 0 {
                data[i] = 6;
                data.push(8);
            } else {
                data[i] -= 1;
            }
        })
    });

    data.len().try_into().unwrap()
}

fn solve_smart(input: &Vec<u8>, day_count: u64) -> u64 {
    let mut fish_count: HashMap<u8, u64> = HashMap::new();
    let mut fish_count_next: HashMap<u8, u64> = HashMap::new();

    for &fish in input {
        *fish_count.entry(fish).or_insert(0) += 1;
    }

    for _ in 0..day_count {
        for (&k, &v) in &fish_count {
            if k == 0 {
                *fish_count_next.entry(8).or_insert(0) += v;
                *fish_count_next.entry(6).or_insert(0) += v;
                continue;
            }

            *fish_count_next.entry(k - 1).or_insert(0) += v;
        }

        swap(&mut fish_count, &mut fish_count_next);
        fish_count_next.clear();
    }

    fish_count.iter().map(|(_, &v)| v).sum()
}
