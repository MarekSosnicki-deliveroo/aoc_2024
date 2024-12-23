use boolinator::Boolinator;
use good_lp::{Solution, SolverModel};
use itertools::Itertools;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 19!");
    let input = read_to_string("inputs/day_19/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut input_split = input.split("\n\n");

    let patterns = input_split
        .next()
        .unwrap()
        .split(", ")
        .sorted_by_key(|p| -(p.len() as i64))
        .collect_vec();

    input_split
        .next()
        .unwrap()
        .split("\n")
        .filter(|design| is_possible_recursive(design.trim(), &patterns))
        .count() as i64
}

fn is_possible_recursive(design: &str, patterns: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }
    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            if is_possible_recursive(&design[pattern.len()..], patterns) {
                return true
            }
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = "r, wr, b, g, bwu, rb, gb, br\n\
\n\
brwrr\n\
bggr\n\
gbbr\n\
rrbgbr\n\
ubwu\n\
bwurrg\n\
brgr\n\
bbrgwb";
        assert_eq!(solve(input), 6);
    }
}
