use itertools::{enumerate, partition, sorted, Itertools};
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::BufRead;
use std::iter;

fn main() {
    println!("Hello day 24!");
    let input = read_to_string("inputs/day_24/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut input_iter = input.split("\n\n");

    let initial_values = input_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|line| sscanf!(line, "{String}: {i64}").unwrap())
        .collect_vec();

    let gates = input_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|line| sscanf!(line, "{String} {String} {String} -> {String}").unwrap())
        .collect_vec();

    let mut evaluated: HashMap<String, i64> = Default::default();

    let mut graph: HashMap<String, GraphLeaf> = Default::default();

    for (node_name, value) in initial_values {
        evaluated.insert(node_name, value);
    }

    let mut to_evaluate = vec![];

    for (l, operator, r, node_name) in gates {
        let node = match operator.as_str() {
            "AND" => GraphLeaf::And(l, r),
            "OR" => GraphLeaf::Or(l, r),
            "XOR" => GraphLeaf::Xor(l, r),
            _ => panic!("Unknown operator {}", operator),
        };
        to_evaluate.push(node_name.clone());
        graph.insert(node_name, node);
    }

    let (mut to_visit, mut to_evaluate): (Vec<String>, Vec<String>) = to_evaluate
        .into_iter()
        .partition(|node| graph.get(node).unwrap().can_evaluate(&evaluated));

    while !to_visit.is_empty() || !to_evaluate.is_empty(){
        if let Some(poped) = to_visit.pop() {
            let value = graph.get(&poped).unwrap().evaluate(&evaluated);
            evaluated.insert(poped, value);
        } else {
            let (new_to_visit, new_to_evaluate) = to_evaluate
                .into_iter()
                .partition(|node| graph.get(node).unwrap().can_evaluate(&evaluated));
            to_visit = new_to_visit;
            to_evaluate = new_to_evaluate;
        }
    }

    // println!("Evaluated {}", evaluated.iter().map(|(n, v)| format!("{n}: {v}")).sorted().join("\n"));

    evaluated
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .sorted()
        .enumerate()
        .map(|(index, (_, value))| value << index)
        .sum()
}

enum GraphLeaf {
    And(String, String),
    Or(String, String),
    Xor(String, String),
}

impl GraphLeaf {
    fn can_evaluate(&self, evaluated: &HashMap<String, i64>) -> bool {
        match self {
            GraphLeaf::And(l, r) | GraphLeaf::Or(l, r) | GraphLeaf::Xor(l, r) => {
                evaluated.contains_key(l) && evaluated.contains_key(r)
            }
        }
    }

    fn evaluate(&self, evaluated: &HashMap<String, i64>) -> i64 {
        match self {
            GraphLeaf::And(l, r) => evaluated.get(l).unwrap() & evaluated.get(r).unwrap(),
            GraphLeaf::Or(l, r) => evaluated.get(l).unwrap() | evaluated.get(r).unwrap(),
            GraphLeaf::Xor(l, r) => {
                if evaluated.get(l) == evaluated.get(r) {
                    0
                } else {
                    1
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "x00: 1\n\
x01: 0\n\
x02: 1\n\
x03: 1\n\
x04: 0\n\
y00: 1\n\
y01: 1\n\
y02: 1\n\
y03: 1\n\
y04: 1\n\
\n\
ntg XOR fgs -> mjb\n\
y02 OR x01 -> tnw\n\
kwq OR kpj -> z05\n\
x00 OR x03 -> fst\n\
tgd XOR rvg -> z01\n\
vdt OR tnw -> bfw\n\
bfw AND frj -> z10\n\
ffh OR nrd -> bqk\n\
y00 AND y03 -> djm\n\
y03 OR y00 -> psh\n\
bqk OR frj -> z08\n\
tnw OR fst -> frj\n\
gnj AND tgd -> z11\n\
bfw XOR mjb -> z00\n\
x03 OR x00 -> vdt\n\
gnj AND wpb -> z02\n\
x04 AND y00 -> kjc\n\
djm OR pbm -> qhw\n\
nrd AND vdt -> hwm\n\
kjc AND fst -> rvg\n\
y04 OR y02 -> fgs\n\
y01 AND x02 -> pbm\n\
ntg OR kjc -> kwq\n\
psh XOR fgs -> tgd\n\
qhw XOR tgd -> z09\n\
pbm OR djm -> kpj\n\
x03 XOR y03 -> ffh\n\
x00 XOR y04 -> ntg\n\
bfw OR bqk -> z06\n\
nrd XOR fgs -> wpb\n\
frj XOR qhw -> z04\n\
bqk OR frj -> z07\n\
y03 OR x01 -> nrd\n\
hwm AND bqk -> z03\n\
tgd XOR rvg -> z12\n\
tnw OR pbm -> gnj";
        assert_eq!(solve(input), 2024);
    }
}
