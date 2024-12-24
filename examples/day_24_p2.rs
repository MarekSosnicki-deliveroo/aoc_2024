use itertools::{enumerate, partition, rev, sorted, Itertools};
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::BufRead;
use std::iter;
use std::iter::zip;
use std::mem::swap;

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

    let mut to_evaluate_initial = vec![];

    for (mut l, operator, mut r, node_name) in gates {
        if l.starts_with("y") {
            swap(&mut l, &mut r);
        }

        let node = match operator.as_str() {
            "AND" => GraphLeaf::And(l, r),
            "OR" => GraphLeaf::Or(l, r),
            "XOR" => GraphLeaf::Xor(l, r),
            _ => panic!("Unknown operator {}", operator),
        };
        to_evaluate_initial.push(node_name.clone());
        graph.insert(node_name, node);
    }

    swap_nodes(&mut graph, "vkq", "z11");
    swap_nodes(&mut graph, "mmk", "z24");
    swap_nodes(&mut graph, "qdq", "pvb");
    swap_nodes(&mut graph, "z38", "hqh");

    let mut suspected_nodes =
        evaluate_graph(evaluated.clone(), &mut graph, to_evaluate_initial.clone());
    println!("Suspected nodes {:?}", suspected_nodes);

    let evaluated_0s: HashMap<String, i64> = evaluated
        .clone()
        .into_iter()
        .map(|(name, value)| (name, 0))
        .collect();
    let evaluated_1s: HashMap<String, i64> = evaluated
        .clone()
        .into_iter()
        .map(|(name, value)| (name, 1))
        .collect();

    suspected_nodes.extend(evaluate_graph(
        evaluated_0s,
        &mut graph,
        to_evaluate_initial.clone(),
    ));
    suspected_nodes.extend(evaluate_graph(
        evaluated_1s,
        &mut graph,
        to_evaluate_initial.clone(),
    ));

    let all_suspected: HashSet<String> = suspected_nodes.into_iter().collect();

    println!("All suspected {:?}", all_suspected);

    for suspected_node in 0..40 {
        let print = print_operations2(&format!("z{suspected_node:02}"), &graph, "");
        println!("{suspected_node}:\n{}", print);

        let no_or = print.split("OR").count() - 1;
        let no_xor = print.split("XOR").count() - 1;
        let no_and = print.split("AND").count() - 1;

        println!("no or {no_or}, no xor: {no_xor}, no_and {no_and}");

        println!(
            "xes:{}",
            (0..40)
                .map(|v| print.split(&format!("x{v:02}")).count() - 1)
                .join(",")
        );
    }

    println!("{}", print_operations("z37", &graph, ""));
    println!("{}", print_operations("z38", &graph, ""));
    println!("{}", print_operations("z39", &graph, ""));


    // SOLVED MANUALLY BY DISPLAYING AND REPLACING NODES WITH ABOVE EXAMPLE!
    let mut swap_result: Vec<_> = vec!["vkq", "z11", "mmk", "z24", "qdq", "pvb", "z38", "hqh"];

    println!("Result is {}", swap_result.iter().sorted().join(","));
    0
}

fn swap_nodes(graph: &mut HashMap<String, GraphLeaf>, l_to_swap: &str, r_to_swap: &str) {
    let l_node = graph.remove(l_to_swap).unwrap();
    let r_node = graph.remove(r_to_swap).unwrap();
    graph.insert(l_to_swap.to_string(), r_node);
    graph.insert(r_to_swap.to_string(), l_node);
}

fn evaluate_graph(
    mut evaluated: HashMap<String, i64>,
    graph: &HashMap<String, GraphLeaf>,
    to_evaluate: Vec<String>,
) -> Vec<String> {
    let (mut to_visit, mut to_evaluate): (Vec<String>, Vec<String>) = to_evaluate
        .into_iter()
        .partition(|node| graph.get(node).unwrap().can_evaluate(&evaluated));

    while !to_visit.is_empty() || !to_evaluate.is_empty() {
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

    let input_x: i64 = evaluated
        .iter()
        .filter(|(name, _)| name.starts_with("x"))
        .sorted()
        .enumerate()
        .map(|(index, (_, value))| value << index)
        .sum();

    let input_y: i64 = evaluated
        .iter()
        .filter(|(name, _)| name.starts_with("y"))
        .sorted()
        .enumerate()
        .map(|(index, (_, value))| value << index)
        .sum();

    let expected_z = input_x + input_y;

    let calculated_z: i64 = evaluated
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .sorted()
        .enumerate()
        .map(|(index, (_, value))| value << index)
        .sum();

    println!("x {input_x}  + y {input_y} = z {expected_z:0b} vs actual_z {calculated_z:0b}");

    let calculated_z_str = format!("{calculated_z:0b}");
    let expected_z_str = format!("{expected_z:0b}");

    let mut suspected_nodes = vec![];
    for (index, (l, r)) in calculated_z_str
        .chars()
        .zip(expected_z_str.chars())
        .enumerate()
    {
        if l != r {
            suspected_nodes.push(format!("z{:2}", calculated_z_str.len() - index - 1))
        }
    }
    suspected_nodes
}

fn print_operations(
    node_name: &str,
    graph: &HashMap<String, GraphLeaf>,
    indentation: &str,
) -> String {
    let inner_indentation = &(indentation.to_string() + " ");
    if let Some(node) = graph.get(node_name) {
        match node {
            GraphLeaf::And(l, r) => format!(
                "{indentation}{node_name}(AND):\n{}\n{}",
                print_operations(l, graph, inner_indentation),
                print_operations(r, graph, inner_indentation)
            ),

            GraphLeaf::Or(l, r) => format!(
                "{indentation}{node_name}(OR):\n{}\n{}",
                print_operations(l, graph, inner_indentation),
                print_operations(r, graph, inner_indentation)
            ),
            GraphLeaf::Xor(l, r) => format!(
                "{indentation}{node_name}(XOR):\n{}\n{}",
                print_operations(l, graph, inner_indentation),
                print_operations(r, graph, inner_indentation)
            ),
        }
    } else {
        format!("{indentation}{node_name}")
    }
}

fn print_operations2(
    node_name: &str,
    graph: &HashMap<String, GraphLeaf>,
    indentation: &str,
) -> String {
    if let Some(node) = graph.get(node_name) {
        match node {
            GraphLeaf::And(l, r) => format!(
                "{indentation}<{node_name}:[{}AND{}]:{node_name}>",
                crate::print_operations2(l, graph, indentation),
                crate::print_operations2(r, graph, indentation)
            ),

            GraphLeaf::Or(l, r) => format!(
                "{indentation}<{node_name}:[{}OR{}]:{node_name}>",
                crate::print_operations2(l, graph, indentation),
                crate::print_operations2(r, graph, indentation)
            ),
            GraphLeaf::Xor(l, r) => format!(
                "{indentation}<{node_name}:[{}XOR{}]:{node_name}>",
                crate::print_operations2(l, graph, indentation),
                crate::print_operations2(r, graph, indentation)
            ),
        }
    } else {
        format!("{indentation}<{node_name}>")
    }
}

fn get_all_possible_faulty_nodes_from(
    node_name: &str,
    graph: &HashMap<String, GraphLeaf>,
) -> Vec<String> {
    if let Some(node) = graph.get(node_name) {
        match node {
            GraphLeaf::And(l, r) | GraphLeaf::Or(l, r) | GraphLeaf::Xor(l, r) => {
                get_all_possible_faulty_nodes_from(l, graph)
                    .into_iter()
                    .chain(get_all_possible_faulty_nodes_from(r, graph))
                    .chain(iter::once(node_name.to_string()))
                    .collect_vec()
            }
        }
    } else {
        vec![]
    }
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
