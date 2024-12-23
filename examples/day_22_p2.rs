use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 22!");
    let input = read_to_string("inputs/day_22/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let sequences_per_client = input
        .split("\n")
        .map(|v| {
            let start_number: i64 = v.parse().unwrap();
            calculate_secret_sequences(start_number)
        })
        .map(|client_sequence| {
            client_sequence
                .into_iter()
                .tuple_windows()
                .map(|((v1, _), (v2, _), (v3, _), (v4, p))| (format!("{v1},{v2},{v3},{v4}"), p))
                .collect_vec()
        })
        .map(|seqeunce_to_prune| {
            let mut already_visited_sequences: HashSet<String> = Default::default();
            println!("Start size {}", seqeunce_to_prune.len());
            let mut result = vec![];
            for (seq, v) in seqeunce_to_prune {
                if !already_visited_sequences.contains(&seq) {
                    already_visited_sequences.insert(seq.clone());
                    result.push((seq, v))
                }
            }
            println!("Final size {}", result.len());
            let result_map: HashMap<String, i64> = result.into_iter().collect();
            result_map
        })
        .collect_vec();

    let value_from_example = sequences_per_client
        .iter()
        .filter_map(|client_seq| client_seq.get("-2,1,-1,3"))
        .sum::<i64>();

    sequences_per_client
        .iter()
        .flat_map(|set| set.keys())
        .unique()
        .map(|seq| {
            (seq, sequences_per_client
                .iter()
                .filter_map(|client_seq| client_seq.get(seq))
                .sum::<i64>())
        })
        .max_by_key(|(_, v)| *v)
        .inspect(|(seq, v)| println!("Best {seq}:{v}"))
        .unwrap().1
}

fn calculate_secret_sequences(start_number: i64) -> Vec<(i64, i64)> {
    const NO_ITERATIONS: usize = 2000;
    let mut result_numbers = vec![start_number];
    let mut number = start_number;
    for _ in 0..NO_ITERATIONS {
        number = (number * 64) ^ number;
        number %= 16777216;
        number = (number / 32) ^ number;
        number %= 16777216;
        number = (number * 2048) ^ number;
        number %= 16777216;
        result_numbers.push(number);
    }
    result_numbers
        .iter()
        .map(|number| number % 10)
        .tuple_windows()
        .map(|(prev, current)| (current - prev, current))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "1\n\
2\n\
3\n\
2024";
        assert_eq!(solve(input), 23);
    }
}
