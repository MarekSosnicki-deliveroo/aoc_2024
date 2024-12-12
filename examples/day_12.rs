use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 12!");
    let input = read_to_string("inputs/day_12/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let map: Vec<Vec<char>> = input
        .split("\n")
        .map(|v_str| v_str.chars().collect_vec())
        .collect_vec();

    let mut result: i64 = 0;

    let mut to_visit_current_region: Vec<Position> = vec![Position { row: 0, column: 0 }];
    let mut current_region_fence_size = 0;
    let mut current_region_size = 0;

    let mut to_visit_other_regions: Vec<Position> = vec![];

    let mut visited: HashSet<Position> = Default::default();

    loop {
        if let Some(to_visit_in_current_region) = to_visit_current_region.pop() {
            if visited.contains(&to_visit_in_current_region) {
                continue;
            }
            visited.insert(to_visit_in_current_region);

            let neighbours = neighbours(&map, &to_visit_in_current_region);

            current_region_size += 1;
            current_region_fence_size += 4 - neighbours.len(); // Add fences for boarders

            let current_region_char = map[to_visit_in_current_region.row][to_visit_in_current_region.column];
            for neighbour in neighbours {
                if current_region_char == map[neighbour.row][neighbour.column] {
                    to_visit_current_region.push(neighbour);
                } else {
                    current_region_fence_size += 1;
                    to_visit_other_regions.push(neighbour);
                }
            }
        } else if let Some(to_visit_in_new_region) = to_visit_other_regions.pop() {
            result += (current_region_size as i64) * (current_region_fence_size as i64);
            current_region_size = 0;
            current_region_fence_size = 0;
            to_visit_current_region.push(to_visit_in_new_region);
        } else {
            break;
        }
    }
    result += (current_region_size as i64) * (current_region_fence_size as i64);

    result
}

fn neighbours(map: &[Vec<char>], position: &Position) -> Vec<Position> {
    let mut neighbours = Vec::new();
    if position.row > 0 {
        neighbours.push(Position {
            row: position.row - 1,
            column: position.column,
        })
    }

    if position.column > 0 {
        neighbours.push(Position {
            row: position.row,
            column: position.column - 1,
        })
    }

    if position.row + 1 < map.len() {
        neighbours.push(Position {
            row: position.row + 1,
            column: position.column,
        })
    }

    if position.column + 1 < map[0].len() {
        neighbours.push(Position {
            row: position.row,
            column: position.column + 1,
        })
    }

    neighbours
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
    fn test_solve() {
        let input = "RRRRIICCFF\n\
RRRRIICCCF\n\
VVRRRCCFFF\n\
VVRCCCJFFF\n\
VVVVCJJCFE\n\
VVIVCCJJEE\n\
VVIIICJJEE\n\
MIIIIIJJEE\n\
MIIISIJEEE\n\
MMMISSJEEE";
        assert_eq!(solve(input), 1930);
    }
}
