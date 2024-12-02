use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 2!");
    let input = read_to_string("inputs/day_02/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            let mut increasing = false;
            let mut decreasing = false;
            for (v1, v2) in line
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .tuple_windows()
            {
                let diff = (v1 - v2).abs();
                if diff < 1 || diff > 3 {
                    return None;
                }
                increasing |= v1 < v2;
                decreasing |= v1 > v2;

                if increasing && decreasing {
                    return None;
                }
            }
            Some(())
        })
        .count() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "7 6 4 2 1\n\
1 2 7 8 9\n\
9 7 6 2 1\n\
1 3 2 4 5\n\
8 6 4 4 1\n\
1 3 6 7 9";
        assert_eq!(solve(input), 2);
    }
}
