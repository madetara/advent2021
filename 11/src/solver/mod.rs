use std::collections::LinkedList;

pub fn solve_1(initial_state: &[[u8; 10]; 10]) -> usize {
    let mut data = initial_state.clone();
    let mut result = 0;

    for _ in 0..100 {
        result += do_step(&mut data);
    }

    result
}

pub fn solve_2(initial_state: &[[u8; 10]; 10]) -> usize {
    let mut data = initial_state.clone();
    let mut result = 1;

    while do_step(&mut data) != 100 {
        result += 1;
    }

    result
}

fn do_step(data: &mut [[u8; 10]; 10]) -> usize {
    let mut flashed = [[false; 10]; 10];
    let mut to_flash = LinkedList::new();
    let mut result = 0;

    for i in 0..10 {
        for j in 0..10 {
            data[i][j] += 1;

            if data[i][j] > 9 {
                to_flash.push_front((i, j));
            }
        }
    }

    while !to_flash.is_empty() {
        let (fx, fy) = to_flash.pop_back().unwrap();

        if flashed[fx][fy] {
            continue;
        }

        data[fx][fy] = 0;
        flashed[fx][fy] = true;
        result += 1;

        for (dx, dy) in coord_deltas() {
            let x = fx as isize + dx;
            let y = fy as isize + dy;

            if x < 0 || x > 9 || y < 0 || y > 9 || flashed[x as usize][y as usize] {
                continue;
            }

            data[x as usize][y as usize] += 1;

            if data[x as usize][y as usize] > 9 {
                to_flash.push_front((x as usize, y as usize));
            }
        }
    }

    result
}

fn coord_deltas() -> Vec<(isize, isize)> {
    vec![
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ]
}
