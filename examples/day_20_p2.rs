use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::{all, enumerate, Itertools};
use sscanf::sscanf;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::read_to_string;
use std::iter;

fn main() {
    println!("Hello day 20!");
    let input = read_to_string("inputs/day_20/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut map = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let (shortest_path_positions, value_map) = dijkstra(&map);
    let shortest_path_len = shortest_path_positions.len();

    // The cost is -1
    println!("Shortest path len {}", shortest_path_len);

    let mut shortcuts: HashSet<(Position, Position, i64)> = Default::default();

    for path_point in shortest_path_positions.iter() {
        shortcuts.extend(find_possible_shortcuts_from(
            &map,
            *path_point,
            &shortest_path_positions,
        ));
    }

    println!("Got {} shortcuts", shortcuts.len());

    let mut result_with_values: BTreeMap<i64, i64> = Default::default();

    for (shortcut_start, shortcut_end, shortcut_cost) in shortcuts.iter() {
        // println!("Analysing shortcut {shortcut_start:?} -> {shortcut_end:?} :  {shortcut_cost}");
        let start_value = value_map.get(shortcut_start).unwrap().0;
        let end_value = value_map.get(shortcut_end).unwrap().0;
        if start_value + shortcut_cost < end_value {
            let cheat_value = end_value - start_value - shortcut_cost;
            *result_with_values.entry(cheat_value).or_default() += 1;
        }
    }

    for (save, value) in result_with_values.iter() {
        println!("For {} positions save {}", value, save);
    }

    result_with_values
        .iter()
        .filter(|(size, count)| **size >= 100)
        .map(|(_, count)| count)
        .sum()
}

fn dijkstra(map: &[Vec<char>]) -> (Vec<Position>, HashMap<Position, (i64, Option<Position>)>) {
    let mut to_visit = vec![find(map, 'S')];
    let mut value_map: HashMap<Position, (i64, Option<Position>)> = Default::default();
    value_map.insert(to_visit[0], (0, None));
    let mut visited: HashSet<Position> = Default::default();
    while let Some(visiting_position) = to_visit.pop() {
        if visited.contains(&visiting_position) {
            continue;
        }
        visited.insert(visiting_position);

        let visiting_time = value_map.get(&visiting_position).unwrap().0;

        let time_after_move = visiting_time + 1;
        for direction in all_directions() {
            if let Some(new_position) = try_move(&map, visiting_position, direction) {
                if !is_blocked(&map, new_position) {
                    let current_follow_value =
                        value_map.entry(new_position).or_insert((i64::MAX, None));
                    if current_follow_value.0 > time_after_move {
                        *current_follow_value = (time_after_move, Some(visiting_position));
                        to_visit.push(new_position);
                    }
                }
            }
        }
        to_visit.sort_by_key(|v| -value_map.get(v).unwrap().0);
    }

    let end_position = find(map, 'E');

    let mut shortest_path_points = vec![end_position];
    let mut to_visit = end_position;
    while let Some(visiting) = value_map.get(&to_visit).unwrap().1 {
        shortest_path_points.push(visiting);
        to_visit = visiting;
    }
    (shortest_path_points, value_map)
}

fn all_directions() -> [Direction; 4] {
    [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
}

fn is_blocked(map: &[Vec<char>], position: Position) -> bool {
    map[position.row][position.column] == '#'
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

fn find_possible_shortcuts_from(
    map: &[Vec<char>],
    start: Position,
    shortest_path: &[Position],
) -> Vec<(Position, Position, i64)> {

    let mut result = vec![];
    for shortest_path_position in shortest_path.iter() {
        if *shortest_path_position != start {
            let distance = (shortest_path_position.column as i64 - start.column as i64).abs() + (shortest_path_position.row as i64 - start.row as i64).abs();

            if distance <= 20 {
                result.push((start, *shortest_path_position, distance))
            }
        }
    }

    result
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

fn print_map(map: &[Vec<char>]) {
    println!("MAP!!");
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if is_blocked(map, Position { row, column }) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!("----");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve_1() {
        let input = "###############\n\
#...#...#.....#\n\
#.#.#.#.#.###.#\n\
#S#...#.#.#...#\n\
#######.#.#.###\n\
#######.#.#...#\n\
#######.#.###.#\n\
###..E#...#...#\n\
###.#######.###\n\
#...###...#...#\n\
#.#####.#.###.#\n\
#.#...#.#.#...#\n\
#.#.#.#.#.#.###\n\
#...#...#...###\n\
###############";
        assert_eq!(solve(input), 0);
    }
}
