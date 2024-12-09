use boolinator::Boolinator;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use gcd::Gcd;

fn main() {
    println!("Hello day 8!");
    let input = read_to_string("inputs/day_08/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let rows_len = map.len();
    let columns_len = map[0].len();

    let mut antennas: HashMap<char, Vec<Position>> = HashMap::default();

    for row in 0..rows_len {
        for column in 0..columns_len {
            if map[row][column] != '.' {
                antennas
                    .entry(map[row][column])
                    .or_default()
                    .push(Position {
                        row: row as i64,
                        column: column as i64,
                    });
            }
        }
    }

    antennas
        .iter()
        .flat_map(|(_, antena_positions)| {
            antena_positions
                .iter()
                .tuple_combinations()
                .flat_map(|(p1, p2)| valid_antinodes(p1, p2, rows_len, columns_len))
        })
        .collect::<HashSet<Position>>()
        .len() as i64
}

fn valid_antinodes(
    p1: &Position,
    p2: &Position,
    rows_len: usize,
    columns_len: usize,
) -> impl Iterator<Item = Position> {


    let gcd =  ((p2.row - p1.row).abs() as u64).gcd((p2.column - p1.column).abs() as u64) as i64;
    let row_diff = (p2.row - p1.row)/ gcd;
    let column_diff = (p2.column - p1.column) /gcd;


    let mut result :Vec<Position> = vec![*p1];

    let mut offset = 1;

    loop {
        let point_to_consider = Position {
            row: p1.row - offset * row_diff,
            column: p1.column - offset * column_diff
        };
        if point_to_consider.is_valid(rows_len, columns_len) {
            result.push(point_to_consider);
            offset +=1;
        } else {
            break
        }
    }

    let mut offset = 1;

    loop {
        let point_to_consider = Position {
            row: p1.row + offset * row_diff,
            column: p1.column + offset * column_diff
        };
        if point_to_consider.is_valid(rows_len, columns_len) {
            println!("Adding point {point_to_consider:?}");

            result.push(point_to_consider);
            offset +=1;
        } else {
            break
        }
    }
    result.into_iter()
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: i64,
    column: i64,
}

impl Position {
    fn is_valid(&self, rows_len: usize, columns_len: usize) -> bool {
        self.row >= 0
            && self.row < rows_len as i64
            && self.column >= 0
            && self.column < columns_len as i64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_t() {
        let input = "T.........\n\
...T......\n\
.T........\n\
..........\n\
..........\n\
..........\n\
..........\n\
..........\n\
..........\n\
..........";
        assert_eq!(solve(input), 9);
    }

    #[test]
    fn test_solve() {
        let input = "............\n\
........0...\n\
.....0......\n\
.......0....\n\
....0.......\n\
......A.....\n\
............\n\
............\n\
........A...\n\
.........A..\n\
............\n\
............";
        assert_eq!(solve(input), 34);
    }
}
