use std::i64::MAX;

pub fn solve_1(input: &Vec<i64>) -> i64 {
    solve(input, |x, d| (x - d).abs())
}

pub fn solve_2(input: &Vec<i64>) -> i64 {
    solve(input, |x, d| {
        let n = (x - d).abs();
        n * (n + 1) / 2
    })
}

fn solve<T>(input: &Vec<i64>, f: T) -> i64
where
    T: Fn(i64, i64) -> i64,
{
    let mut result = MAX;
    let &min = input.iter().min().unwrap();
    let &max = input.iter().max().unwrap();

    for d in min..=max {
        let mut current = 0;

        for &x in input {
            current += f(x, d);

            if current > result {
                break;
            }
        }

        if current < result {
            result = current;
        }
    }

    result
}
