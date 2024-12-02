extern crate core;

use std::collections::HashSet;
use itertools::{enumerate, Itertools};
use std::fs::read_to_string;
use std::iter;

fn main() {
    println!("Hello day 2!");
    let input = read_to_string("inputs/day_02/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    input
        .lines()
        .filter_map(|line| {
            println!("--- Analysing line {}", line);
            // First check if the whole thing is valid
            let values = line
                .split(" ")
                .map(|v| v.parse::<i64>().unwrap())
                .collect_vec();

            if is_line_ok(&values) {
                println!("Line ok from the start");
                return Some(())
            }
            let mut increasing_indexes: HashSet<usize> = HashSet::default();
            let mut decreasing_indexes: HashSet<usize>  = HashSet::default();
            let mut to_try_to_remove: HashSet<usize>  = HashSet::default();
            for (index, (v1, v2)) in values.iter().tuple_windows().enumerate() {
                if is_bad_diff(*v1, *v2) {
                    to_try_to_remove.insert(index);
                    to_try_to_remove.insert(index + 1);
                }
                if v1 > v2 {
                    decreasing_indexes.insert(index);
                    decreasing_indexes.insert(index+1);
                } else {
                    increasing_indexes.insert(index);
                    increasing_indexes.insert(index+1);
                }
            }
            if increasing_indexes.len() > decreasing_indexes.len() {
                to_try_to_remove.extend(decreasing_indexes);
            } else {
                to_try_to_remove.extend(increasing_indexes)
            }

            for index_to_remove in to_try_to_remove {
                let mut values_clone = values.clone();
                values_clone.remove(index_to_remove);
                if is_line_ok(&values_clone) {
                    println!("Line is ok if modified to {:?}", values_clone);
                    return  Some(())
                }
            }
            return None
        })
        .count() as i64
}

fn is_bad_diff(v1: i64, v2: i64) -> bool {
    let diff = (v1 - v2).abs();
    diff < 1 || diff > 3
}

fn is_line_ok(values: &[i64]) -> bool {
    let mut increasing = false;
    let mut decreasing = false;
    for (v1, v2) in values.iter().tuple_windows() {
        let diff = (v1 - v2).abs();
        if diff < 1 || diff > 3 {
            return false;
        }
        increasing |= v1 < v2;
        decreasing |= v1 > v2;

        if increasing && decreasing {
            return false;
        }
    }
    true
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
        assert_eq!(solve(input), 4);
    }

    #[test]
    fn test_solve_edge() {
        let input = "0 6 4 2 1\n\
1 3 6 7 9 100";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_solve_edge_with_two() {
        let input = "1 3 100 5 6 100";
        assert_eq!(solve(input), 0);
    }
}
