use std::collections::{HashSet, LinkedList};

pub fn solve_1(data: &Vec<Vec<u8>>) -> usize {
    find_low_points(data)
        .iter()
        .map(|&(i, j)| data[i][j] as usize + 1)
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}

pub fn solve_2(data: &Vec<Vec<u8>>) -> usize {
    let mut basin_sizes = vec![];

    for &(x, y) in find_low_points(data).iter() {
        let mut size = 0;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut q = LinkedList::new();
        q.push_front((x, y));

        while !q.is_empty() {
            let (i, j) = q.pop_back().unwrap();
            if visited.contains(&(i, j)) {
                continue;
            }

            size += 1;
            visited.insert((i, j));

            for &(di, dj) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter() {
                let ii = i as isize + di;
                let jj = j as isize + dj;

                if ii < 0
                    || ii as usize >= data.len()
                    || jj < 0
                    || jj as usize >= data[i].len()
                    || data[ii as usize][jj as usize] <= data[i][j]
                    || visited.contains(&(ii as usize, jj as usize))
                    || data[ii as usize][jj as usize] == 9
                {
                    continue;
                }

                q.push_front((ii as usize, jj as usize));
            }
        }

        basin_sizes.push(size);
    }

    basin_sizes.sort();
    basin_sizes.reverse();

    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn find_low_points(data: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut result = vec![];

    for (i, v) in data.iter().enumerate() {
        for (j, &x) in v.iter().enumerate() {
            if [(1, 0), (-1, 0), (0, 1), (0, -1)].iter().all(|(di, dj)| {
                i as isize + di < 0
                    || j as isize + dj < 0
                    || i as isize + di >= data.len() as isize
                    || j as isize + dj >= data[i].len() as isize
                    || data[(i as isize + di) as usize][(j as isize + dj) as usize] > x
            }) {
                result.push((i, j));
            }
        }
    }

    result
}
