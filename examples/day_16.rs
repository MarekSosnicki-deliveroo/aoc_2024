use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 16!");
    let input = read_to_string("inputs/day_16/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut map = input
        .split("\n")
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut to_visit = vec![(find(&map, 'S'), Direction::Right)];
    let mut value_map: HashMap<(Position, Direction), i64> = Default::default();

    value_map.insert(to_visit[0], 0);

    let mut visited: HashSet<(Position, Direction)> = Default::default();

    while let Some((visiting, current_direction)) = to_visit.pop() {
        if visited.contains(&(visiting, current_direction)) {
            continue;
        }
        visited.insert((visiting, current_direction));

        let visiting_value = *value_map.get(&(visiting, current_direction)).unwrap();

        if let Some(follow_point) = try_move(&map, visiting, current_direction) {
            if map[follow_point.row][follow_point.column] != '#' {
                let current_follow_value = value_map
                    .entry((follow_point, current_direction))
                    .or_insert(i64::MAX);
                *current_follow_value = (visiting_value + 1).min(*current_follow_value);
                to_visit.push((follow_point, current_direction))
            }
        }

        {
            let rotated_right = current_direction.rotate_right();

            let current_follow_value = value_map
                .entry((visiting, rotated_right))
                .or_insert(i64::MAX);
            *current_follow_value = (visiting_value + 1000).min(*current_follow_value);
            to_visit.push((visiting, rotated_right))
        }

        {
            let rotated_left = current_direction.rotate_left();
            let current_follow_value = value_map
                .entry((visiting, rotated_left))
                .or_insert(i64::MAX);
            *current_follow_value = (visiting_value + 1000).min(*current_follow_value);
            to_visit.push((visiting, rotated_left))
        }

        to_visit.sort_by_key(|v| -value_map.get(v).unwrap());
    }

    let end = find(&map, 'E');

    [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ]
    .iter()
    .filter_map(|dir| value_map.get(&(end, *dir)))
    .min()
    .cloned()
    .unwrap()
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

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
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

fn print_map(map: &[Vec<char>]) {
    println!("MAP!!");
    println!("{}", map.iter().map(|row| row.iter().join("")).join("\n"));
    println!("----");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = "###############\n\
#.......#....E#\n\
#.#.###.#.###.#\n\
#.....#.#...#.#\n\
#.###.#####.#.#\n\
#.#.#.......#.#\n\
#.#.#####.###.#\n\
#...........#.#\n\
###.#.#####.#.#\n\
#...#.....#.#.#\n\
#.#.#.###.#.#.#\n\
#.....#...#.#.#\n\
#.###.#.#.#.#.#\n\
#S..#.....#...#\n\
###############";
        assert_eq!(solve(input), 7036);
    }

    #[test]
    fn test_solve_2() {
        let input = "#################\n\
#...#...#...#..E#\n\
#.#.#.#.#.#.#.#.#\n\
#.#.#.#...#...#.#\n\
#.#.#.#.###.#.#.#\n\
#...#.#.#.....#.#\n\
#.#.#.#.#.#####.#\n\
#.#...#.#.#.....#\n\
#.#.#####.#.###.#\n\
#.#.#.......#...#\n\
#.#.###.#####.###\n\
#.#.#...#.....#.#\n\
#.#.#.#####.###.#\n\
#.#.#.........#.#\n\
#.#.#.#########.#\n\
#S#.............#\n\
#################";
        assert_eq!(solve(input), 11048);
    }
}
