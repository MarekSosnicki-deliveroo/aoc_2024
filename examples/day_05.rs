use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

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

    split.next().unwrap().split("\n").map(|line| {
        let values: Vec<i64> = line.split(",").map(|v| v.parse().unwrap()).collect();

        let mut visted : HashSet<i64> = HashSet::default();

        for value in values.iter().skip(0) {
            // println!("Analysing value {value}");
            if let Some(rules) = should_be_before.get(value) {
                // println!("Rules are {:?}", rules);
                // println!("Visited are {:?}", visted);
                if rules.iter().any(|rule_v| visted.contains(rule_v) ) {
                    println!("Values {:?} are not ok ", values);
                    return 0;
                }
            }
            visted.insert(*value);
        }
        println!("Values {:?} are ok ", values);
        println!("Adding value {}", values[values.len()/2]);
        return values[values.len()/2]
    }).sum()
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
        assert_eq!(solve(input), 143);
    }
}
