use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 6!");
    let input = read_to_string("inputs/day_06/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut current_position = find_start(&map);
    let mut current_direction = Direction::Up;
    let mut visited: HashSet<Position> = HashSet::default();
    visited.insert(current_position);
    while let Some(new_position) = try_move(&map, current_position, current_direction) {
        if is_valid(&map, new_position) {
            current_position = new_position;
            visited.insert(new_position);
        } else {
            current_direction = current_direction.rotate_right()
        }
    }

    visited.len() as i64
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Copy, Clone)]
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
}

fn find_start(map: &[Vec<char>]) -> Position {
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '^' {
                return Position { row, column };
            }
        }
    }
    panic!("No starting position for guard found found")
}

fn is_valid(map: &[Vec<char>], p: Position) -> bool {
    map[p.row][p.column] != '#'
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
        let input = "....#.....\n\
.........#\n\
..........\n\
..#.......\n\
.......#..\n\
..........\n\
.#..^.....\n\
........#.\n\
#.........\n\
......#...";
        assert_eq!(solve(input), 41);
    }
}
