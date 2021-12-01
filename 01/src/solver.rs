pub fn solve_part1(input: &[u64]) -> u64 {
    let mut result_counter: u64 = 0;

    input.iter().enumerate().for_each(|(i, x)| {
        if i > 0 && x > &input[i - 1] {
            result_counter += 1;
        }
    });

    result_counter
}

pub fn solve_part2(input: &[u64]) -> u64 {
    let mut result_counter: u64 = 0;

    input.iter().enumerate().for_each(|(i, _)| {
        if i < 3 {
            return;
        }

        let previous_window = input[i - 1] + input[i - 2] + input[i - 3];
        let current_window = input[i] + input[i - 1] + input[i - 2];

        if current_window > previous_window {
            result_counter += 1;
        }
    });

    result_counter
}
