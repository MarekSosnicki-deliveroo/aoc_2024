use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 18!");
    let input = read_to_string("inputs/day_18/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim(), 71, 71, 1024),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str, rows:usize, columns: usize, simulate_time: i64) -> i64 {
    let mut map : Vec<Vec<Option<i64>>> = vec![vec![None; columns]; rows];

    let mut time = 1;
    for (row, column) in input.split("\n").map(|line| sscanf!(line, "{usize},{usize}").unwrap()) {
        map[column][row] = Some(time);
        time += 1;
    }

    let mut to_visit = vec![Position{
        row: 0,
        column: 0
    }];
    let mut value_map: HashMap<Position, i64> = Default::default();
    value_map.insert(to_visit[0], 0);

    let mut visited: HashSet<Position> = Default::default();


    while let Some(visiting_position) = to_visit.pop() {
        if visited.contains(&visiting_position) {
            continue;
        }
        visited.insert(visiting_position);

        let visiting_time = *value_map.get(&visiting_position).unwrap();

        let time_after_move = visiting_time + 1;
        for direction in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            if let Some(new_position) = try_move(&map, visiting_position, direction) {
                if !blocked_at(&map, new_position, simulate_time) {
                    let current_follow_value = value_map.entry(new_position).or_insert(i64::MAX);

                    *current_follow_value = time_after_move.min(*current_follow_value);
                    to_visit.push(new_position);
                }


            }
        }
        to_visit.sort_by_key(|v| -value_map.get(v).unwrap());
    }

    *value_map.get(&Position{
        row: map.len() -1,
        column: map[0].len() -1
    }).unwrap()
}


fn blocked_at(map: &[Vec<Option<i64>>], position: Position, time: i64) -> bool {
    map[position.row][position.column].map(|v| v <= time ).unwrap_or_default()
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

fn try_move(map: &[Vec<Option<i64>>], start: Position, direction: Direction) -> Option<Position> {
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

fn print_map(map: &[Vec<Option<i64>>], at: i64) {
    println!("MAP!!");
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if blocked_at(map, Position{
                row,
                column,
            }, at) {
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
        let input = "5,4\n\
4,2\n\
4,5\n\
3,0\n\
2,1\n\
6,3\n\
2,4\n\
1,5\n\
0,6\n\
3,3\n\
2,6\n\
5,1\n\
1,2\n\
5,5\n\
2,5\n\
6,5\n\
1,4\n\
0,4\n\
6,4\n\
1,1\n\
6,1\n\
1,0\n\
0,5\n\
1,6\n\
2,0";
        assert_eq!(solve(input, 7, 7, 12), 22);
    }
}
