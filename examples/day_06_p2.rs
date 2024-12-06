use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 6!");
    let input = read_to_string("inputs/day_06/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {}, time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );

    let start_solve2 = std::time::Instant::now();
    println!(
        "Solve_v2 result is {}, time {}ms",
        solve_v2(input.trim()),
        start_solve2.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut valid_block_position_count: i64 = 0;

    let visited = visited_or_loop(&map).unwrap();

    for visited_position in visited {
        map[visited_position.row][visited_position.column] = '#';

        if visited_or_loop(&map).is_err() {
            valid_block_position_count += 1;
        }

        map[visited_position.row][visited_position.column] = '.';
    }

    valid_block_position_count
}

// Result Ok -> list of visited except start, Err -> loop found
fn visited_or_loop(map: &[Vec<char>]) -> Result<HashSet<Position>, ()> {
    let start_position = find_start(&map);
    let mut current_position = find_start(&map);
    let mut current_direction = Direction::Up;
    let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::default();
    visited
        .entry(current_position)
        .or_default()
        .insert(current_direction);
    while let Some(new_position) = try_move(&map, current_position, current_direction) {
        if is_valid(&map, new_position) {
            current_position = new_position;

            if visited
                .entry(new_position)
                .or_default()
                .contains(&current_direction)
            {
                return Err(());
            }

            visited
                .entry(new_position)
                .or_default()
                .insert(current_direction);
        } else {
            current_direction = current_direction.rotate_right()
        }
    }
    visited.remove(&start_position);
    Ok(visited.keys().cloned().collect())
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

fn solve_v2(input: &str) -> i64 {
    let map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut valid_block_position_count: i64 = 0;
    let start_position = find_start(&map);
    let mut current_position = find_start(&map);
    let mut current_direction = Direction::Up;
    let mut visited: HashMap<Position, HashSet<Direction>> = HashMap::default();
    visited
        .entry(current_position)
        .or_default()
        .insert(current_direction);

    while let Some(new_position) = try_move(&map, current_position, current_direction) {
        if is_valid(&map, new_position) {
            if !visited.contains_key(&new_position) {
                if is_loop_from(
                    &map,
                    &visited,
                    current_position,
                    current_direction,
                    new_position,
                ) {
                    valid_block_position_count += 1;
                }
            }
            current_position = new_position;
            visited
                .entry(new_position)
                .or_default()
                .insert(current_direction);
        } else {
            current_direction = current_direction.rotate_right()
        }
    }
    visited.remove(&start_position);

    valid_block_position_count
}

fn is_loop_from(
    map: &[Vec<char>],
    visited_before: &HashMap<Position, HashSet<Direction>>,
    start_position: Position,
    start_direction: Direction,
    additional_blockade: Position,
) -> bool {
    let mut current_position = start_position;
    let mut current_direction = start_direction;
    let mut visited_in_check: HashMap<Position, HashSet<Direction>> = HashMap::default();
    while let Some(new_position) = try_move(&map, current_position, current_direction) {
        if is_valid(&map, new_position) && new_position != additional_blockade {
            current_position = new_position;

            if visited_before
                .get(&new_position)
                .map(|directions| directions.contains(&current_direction))
                .unwrap_or(false)
                || visited_in_check
                    .entry(new_position)
                    .or_default()
                    .contains(&current_direction)
            {
                return true;
            }

            visited_in_check
                .entry(new_position)
                .or_default()
                .insert(current_direction);
        } else {
            current_direction = current_direction.rotate_right()
        }
    }
    false
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
        assert_eq!(solve_v2(input), 6);
    }
}
