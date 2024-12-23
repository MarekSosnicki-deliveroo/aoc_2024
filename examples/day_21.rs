use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fmt::format;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 21!");
    let input = read_to_string("inputs/day_21/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut cache: HashMap<(char, char), Vec<String>> = Default::default();
    input
        .split("\n")
        .map(|code| {
            println!("Parsing {}", code);
            let code_numeric: i64 = code[0..code.len() - 1].parse().unwrap();
            let solution = solve_for_code(code, &mut cache);
            code_numeric * solution
        })
        .sum()
}

fn solve_for_code(code: &str, cache: &mut HashMap<(char, char), Vec<String>>) -> i64 {
    let numeric_keyboard = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['x', '0', 'A'],
    ];

    let robot_keyboard = vec![vec!['x', '^', 'A'], vec!['<', 'v', '>']];

    let numeric_keyboard_sequences = filter_shortest(shortest_sequences_for_keyboard(
        code,
        &numeric_keyboard,
        cache,
    ));

    println!("numeric_keyboard_sequence: {numeric_keyboard_sequences:?}");

    let first_robot_sequences = filter_shortest(
        numeric_keyboard_sequences
            .into_iter()
            .flat_map(|s| shortest_sequences_for_keyboard(&s, &robot_keyboard, cache))
            .collect_vec(),
    );

    println!("first_robot_sequences: {first_robot_sequences:?}");

    let final_sequences = filter_shortest(
        first_robot_sequences
            .into_iter()
            .flat_map(|s| shortest_sequences_for_keyboard(&s, &robot_keyboard, cache))
            .collect_vec(),
    );

    println!("final_sequence: {final_sequences:?}");
    println!("Final sequence len: {}", final_sequences[0].len());

    final_sequences[0].len() as i64
}

fn shortest_sequences_for_keyboard(
    code: &str,
    keyboard: &[Vec<char>],
    cache: &mut HashMap<(char, char), Vec<String>>,
) -> Vec<String> {
    let a_position = find(&keyboard, 'A');
    let mut position = a_position;

    let mut possible_sequences : HashSet<String> = Default::default();

    for code_character in code.chars() {
        let current_char = keyboard[position.row][position.column];
        let char_position = find(&keyboard, code_character);

        let move_possibilities = cache
            .entry((current_char, code_character))
            .or_insert_with(|| moves_from_to(&keyboard, position, char_position));

        if possible_sequences.is_empty() {
            possible_sequences = move_possibilities.iter().cloned().collect();
        } else {
            possible_sequences = possible_sequences
                .into_iter()
                .flat_map(|sequence| {
                    move_possibilities
                        .iter()
                        .map(|move_possibility| format!("{sequence}{move_possibility}A"))
                        .collect_vec()
                })
                .collect();
        }

        position = char_position;

        // println!("Possible sequences {:?}", possible_sequences);
    }
    possible_sequences.into_iter().collect()
}

fn moves_from_to(keyboard: &[Vec<char>], start: Position, end: Position) -> Vec<String> {
    let mut result = vec![];
    if start.row > end.row {
        result.extend(vec!['^'; (start.row - end.row)].into_iter());
    } else {
        result.extend(vec!['v'; (end.row - start.row)].into_iter());
    }

    if start.column > end.column {
        result.extend(vec!['<'; (start.column - end.column)].into_iter());
    } else {
        result.extend(vec!['>'; (end.column - start.column)].into_iter());
    }

    if keyboard[0][0] == '7' {
        if start.row == 3 && end.column == 0 {
            return vec![result.clone().into_iter().join("")];
        }

        if end.row == 3 && start.column == 0 {
            return vec![result.clone().into_iter().rev().join("")];
        }
    }

    if keyboard[0][0] == 'x' {
        if start.row == 1 && start.column == 0 && end.row == 0 {
            return vec![result.clone().into_iter().rev().join("")];
        }

        if end.row == 1 && end.column == 0 && start.row == 0 {
            return vec![result.clone().into_iter().join("")];
        }
    }

    vec![
        result.clone().into_iter().join(""),
        result.into_iter().rev().join(""),
    ]
}

fn filter_shortest(sequences: Vec<String>) -> Vec<String> {
    let shortest = sequences.iter().map(|s| s.len()).min().unwrap();

    sequences
        .into_iter()
        .filter(|s| s.len() == shortest)
        .unique()
        .collect()
}

fn find(map: &[Vec<char>], value: char) -> Position {
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == value {
                return Position { row, column };
            }
        }
    }
    panic!("No starting position found")
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

fn try_move(map: &[Vec<char>], start: Position, direction: Direction) -> Option<Position> {
    match direction {
        Direction::Up => {
            if start.row == 0 {
                None
            } else {
                Some(Position {
                    row: start.row - 1,
                    column: start.column,
                })
            }
        }
        Direction::Right => {
            if start.column + 1 == map[start.row].len() {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column + 1,
                })
            }
        }
        Direction::Down => {
            if start.row + 1 == map.len() {
                None
            } else {
                Some(Position {
                    row: start.row + 1,
                    column: start.column,
                })
            }
        }
        Direction::Left => {
            if start.column == 0 {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column - 1,
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "029A\n\
980A\n\
179A\n\
456A\n\
379A";
        assert_eq!(solve(input), 126384);
    }
}
