use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 23!");
    let input = read_to_string("inputs/day_23/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let connections = input
        .split("\n")
        .map(|line| sscanf!(line, "{String}-{String}").unwrap())
        .collect_vec();

    let mut graph : HashMap<String, HashSet<String>> = Default::default();

    for (l, r) in connections {
        graph.entry(l.clone()).or_default().insert(r.clone());
        graph.entry(r.clone()).or_default().insert(l.clone());
    }

    let mut possible_triples : HashSet<Vec<&String>> = Default::default();

    for (node_name, connections) in graph.iter() {
        if node_name.starts_with("t") {
            for (c1, c2) in connections.iter().tuple_combinations() {
                if graph.get(c1).unwrap().contains(c2) {
                    possible_triples.insert(
                        [node_name, c1, c2].into_iter().sorted().collect()
                    );
                }
            }
        }
    }
    possible_triples.len() as i64
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "kh-tc\n\
qp-kh\n\
de-cg\n\
ka-co\n\
yn-aq\n\
qp-ub\n\
cg-tb\n\
vc-aq\n\
tb-ka\n\
wh-tc\n\
yn-cg\n\
kh-ub\n\
ta-co\n\
de-co\n\
tc-td\n\
tb-wq\n\
wh-td\n\
ta-ka\n\
td-qp\n\
aq-cg\n\
wq-ub\n\
ub-vc\n\
de-ta\n\
wq-aq\n\
wq-vc\n\
wh-yn\n\
ka-de\n\
kh-ta\n\
co-tc\n\
wh-qp\n\
tb-vc\n\
td-yn";
        assert_eq!(solve(input), 7);
    }
}
