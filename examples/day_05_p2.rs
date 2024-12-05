use itertools::{enumerate, Itertools};
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::iter;

fn main() {
    println!("Hello day 5!");
    let input = read_to_string("inputs/day_05/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    let mut split = input.split("\n\n");
    let should_be_before: HashMap<i64, Vec<i64>> =
        split
            .next()
            .unwrap()
            .split("\n")
            .fold(HashMap::default(), |mut result, values_str| {
                let (l, r) = sscanf!(values_str, "{i64}|{i64}").unwrap();
                result.entry(l).or_default().push(r);
                result
            });

    split
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut values: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();

            let mut visted: HashSet<i64> = HashSet::default();

            let mut was_fixed = false;

            for index in 0..values.len() {
                let visiting_value = values[index];

                if let Some(rules) = should_be_before.get(&visiting_value) {
                    if rules.iter().any(|rule_v| visted.contains(rule_v)) {
                        was_fixed = true;
                        println!("Values {:?} are not ok ", values);
                        let mut to_fix = values.clone();
                        let to_fix_value = to_fix.remove(index);
                        println!("Fixing value {to_fix_value}");
                        println!("To fix {:?}", to_fix);

                        for to_fix_index in 0..to_fix.len() {
                            if rules.iter().any(|rule_value| *rule_value == to_fix[to_fix_index]) {
                                to_fix.insert(to_fix_index, to_fix_value);
                                break;
                            }
                        }
                        if to_fix.len() < values.len() {
                            to_fix.push(to_fix_value);
                        }
                        println!("After fix {:?}", to_fix);
                        values = to_fix;
                    }
                }
                visted.insert(visiting_value);
            }
            if was_fixed {
                values[values.len()/2]
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "47|53\n\
97|13\n\
97|61\n\
97|47\n\
75|29\n\
61|13\n\
75|53\n\
29|13\n\
97|29\n\
53|29\n\
61|53\n\
97|53\n\
61|29\n\
47|13\n\
75|47\n\
97|75\n\
47|61\n\
75|61\n\
47|29\n\
75|13\n\
53|13\n\
\n\
75,47,61,53,29\n\
97,61,53,29,13\n\
75,29,13\n\
75,97,47,61,53\n\
61,13,29\n\
97,13,75,29,47";
        assert_eq!(solve(input), 123);
    }
}
