use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
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

fn solve(input: &str) -> String {
    let connections = input
        .split("\n")
        .map(|line| sscanf!(line, "{String}-{String}").unwrap())
        .collect_vec();

    let mut graph: HashMap<String, HashSet<String>> = Default::default();

    for (l, r) in connections {
        graph.entry(l.clone()).or_default().insert(r.clone());
        graph.entry(r.clone()).or_default().insert(l.clone());
    }

    let mut possible_triples: HashSet<Vec<&String>> = Default::default();

    for (node_name, connections) in graph.iter() {
        for (c1, c2) in connections.iter().tuple_combinations() {
            if graph.get(c1).unwrap().contains(c2) {
                possible_triples.insert([node_name, c1, c2].into_iter().sorted().collect());
            }
        }
    }

    let mut biggest_group: Vec<&String> = vec![];

    for possible_triple in possible_triples {
        let first_node_connections = graph.get(possible_triple[0]).unwrap();
        let mut result_group: HashSet<&String> = possible_triple.into_iter().collect();
        for connection in first_node_connections {
            if !result_group.contains(connection) {
                let connection_connections = graph.get(connection).unwrap();
                if result_group
                    .iter()
                    .all(|result_conn| connection_connections.contains(*result_conn))
                {
                    result_group.insert(connection);
                }
            }
        }

        if result_group.len() > biggest_group.len() {
            biggest_group = result_group.into_iter().collect();
        }
    }
    biggest_group.into_iter().sorted().join(",")
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
        assert_eq!(solve(input), "co,de,ka,ta");
    }
}
