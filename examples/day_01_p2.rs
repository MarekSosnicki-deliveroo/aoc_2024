use std::collections::HashMap;
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 1! part 2");
    let input = read_to_string("inputs/day_01/input").unwrap();

    println!("Result is {}", solve(&input))
}

fn solve(input: &str) -> i64 {
    let left = input
        .trim()
        .split("\n")
        .map(|ids| sscanf!(ids, "{i64}   {i64}").unwrap().0)
        .collect_vec();

    let right : HashMap<i64, i64> = input
        .trim()
        .split("\n")
        .map(|ids| sscanf!(ids, "{i64}   {i64}").unwrap().1)
        .sorted()
        .chunk_by(|v| *v)
        .into_iter()
        .map(|(v, group)| (v, group.count() as i64))
        .collect();

    left.iter().map(|v| right.get(v).cloned().unwrap_or_default() * v).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "3   4\n\
4   3\n\
2   5\n\
1   3\n\
3   9\n\
3   3";
        assert_eq!(solve(input), 31);
    }
}
