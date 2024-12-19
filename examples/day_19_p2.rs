use itertools::Itertools;
use std::collections::{HashMap};
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

    let mut cache = Default::default();

    input_split
        .next()
        .unwrap()
        .split("\n")
        .map(|design| {
            count_options_recursive(design.trim(), &patterns, &mut cache)
        })
        .sum()
}

fn count_options_recursive(design: &str, patterns: &[&str], cache : &mut HashMap<String, i64>) -> i64 {
    if let Some(value) = cache.get(design) {
        return *value;
    }
    if design.is_empty() {
        return 1;
    }
    let mut result = 0;
    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            result += count_options_recursive(&design[pattern.len()..], patterns, cache);
        }
    }

    cache.insert(design.to_string(), result);

    result
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
        assert_eq!(solve(input), 16);
    }
}
