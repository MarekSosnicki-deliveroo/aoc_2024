use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
// For part 1
// const NO_OF_BLINKS: i64 = 25;

// For part 2
const NO_OF_BLINKS: i64 = 75;

fn main() {
    println!("Hello day 11!");
    let input = read_to_string("inputs/day_11/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let numbers: Vec<i64> = input
        .split(" ")
        .map(|v_str| v_str.parse().unwrap())
        .collect_vec();

    let mut cache: HashMap<(i64, i64), i64> = HashMap::default();

    numbers
        .into_iter()
        .map(|n| count_after_n_iterations(n, NO_OF_BLINKS, &mut cache))
        .sum()
}

fn count_after_n_iterations(
    start_number: i64,
    no_of_iterations_left: i64,
    cache: &mut HashMap<(i64, i64), i64>,
) -> i64 {
    if no_of_iterations_left == 0 {
        return 1;
    }
    if let Some(v) = cache.get(&(start_number, no_of_iterations_left)).cloned() {
        v
    } else {
        let v = if start_number == 0 {
            count_after_n_iterations(1, no_of_iterations_left - 1, cache)
        } else {
            let start_number_str = start_number.to_string();
            if start_number_str.len() % 2 == 0 {
                let left: i64 = start_number_str[0..(start_number_str.len() / 2)]
                    .parse()
                    .unwrap();
                let right: i64 = start_number_str[(start_number_str.len() / 2)..]
                    .parse()
                    .unwrap();
                count_after_n_iterations(left, no_of_iterations_left - 1, cache)
                    + count_after_n_iterations(right, no_of_iterations_left - 1, cache)
            } else {
                count_after_n_iterations(start_number * 2024, no_of_iterations_left - 1, cache)
            }
        };

        cache.insert((start_number, no_of_iterations_left), v);
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "125 17";
        assert_eq!(solve(input), 55312);
    }
}
