use std::fs;

pub fn read_input(filename: &str) -> Vec<((u64, u64), (u64, u64))> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(|s| {
            let (a, b) = {
                let temp: Vec<&str> = s.split(" -> ").collect();
                (temp[0], temp[1])
            };

            let (a_x, a_y) = {
                let temp: Vec<&str> = a.split(',').collect();
                (temp[0], temp[1])
            };

            let (b_x, b_y) = {
                let temp: Vec<&str> = b.split(',').collect();
                (temp[0], temp[1])
            };

            (
                (a_x.parse().unwrap(), a_y.parse().unwrap()),
                (b_x.parse().unwrap(), b_y.parse().unwrap()),
            )
        })
        .collect()
}
