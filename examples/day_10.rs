use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 10!");
    let input = read_to_string("inputs/day_10/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - '0' as u8).collect_vec())
        .collect_vec();

    let mut start_positions: Vec<Position> = vec![];

    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == 0 {
                start_positions.push(Position { row, column });
            }
        }
    }

    start_positions
        .iter()
        .map(|p| all_trails_from_start(&map, *p).len() as i64)
        .sum()
}

fn all_trails_from_start(map: &[Vec<u8>], start: Position) -> HashSet<Position> {
    all_trails_recursive(map, start, 0).into_iter().collect()
}

fn all_trails_recursive(map: &[Vec<u8>], p: Position, expected_value: u8) -> Vec<Position> {
    let p_value = map[p.row][p.column];
    if expected_value != p_value {
        return vec![];
    }

    if map[p.row][p.column] == 9 {
        return vec![p];
    }

    let mut result = vec![];

    if p.row > 0 {
        result.extend(all_trails_recursive(
            map,
            Position {
                row: p.row - 1,
                column: p.column,
            },
            expected_value + 1,
        ));
    }

    if p.row + 1 < map.len() {
        result.extend(all_trails_recursive(
            map,
            Position {
                row: p.row + 1,
                column: p.column,
            },
            expected_value + 1,
        ));
    }

    if p.column > 0 {
        result.extend(all_trails_recursive(
            map,
            Position {
                row: p.row,
                column: p.column - 1,
            },
            expected_value + 1,
        ));
    }


    if p.column+1 < map[0].len() {
        result.extend(all_trails_recursive(
            map,
            Position {
                row: p.row,
                column: p.column + 1,
            },
            expected_value + 1,
        ));
    }
    result
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "0123\n\
1234\n\
8765\n\
9876";
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn test_simple2() {
        let input = "10aa9aa\n\
2aaa8aa\n\
3aaa7aa\n\
4567654\n\
aaa8aa3\n\
aaa9aa2\n\
aaaaa01";
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_solve() {
        let input = "89010123\n\
78121874\n\
87430965\n\
96549874\n\
45678903\n\
32019012\n\
01329801\n\
10456732";
        assert_eq!(solve(input), 36);
    }
}
