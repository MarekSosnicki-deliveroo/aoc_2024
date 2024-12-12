use itertools::Itertools;
use std::collections::{HashSet};
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
    let mut current_region_fences : Vec<(Position, Position)> = Vec::new();
    let mut current_region_size = 0;

    let mut to_visit_other_regions: Vec<Position> = vec![];

    let mut visited: HashSet<Position> = Default::default();

    loop {
        if let Some(position_in_current_region) = to_visit_current_region.pop() {
            if visited.contains(&position_in_current_region) {
                continue;
            }
            visited.insert(position_in_current_region);

            let neighbours = neighbours(&position_in_current_region);

            current_region_size += 1;

            let current_region_char =
                map[position_in_current_region.row as usize][position_in_current_region.column as usize];
            for neighbour in neighbours {
                if let Some(neighbour_value) = neighbour_value(&map, &neighbour) {
                    if current_region_char == neighbour_value {
                        to_visit_current_region.push(neighbour);
                    } else {
                        to_visit_other_regions.push(neighbour);
                        current_region_fences.push((position_in_current_region, neighbour))
                    }

                } else {
                    current_region_fences.push((position_in_current_region, neighbour))
                }

            }
        } else if let Some(to_visit_in_new_region) = to_visit_other_regions.pop() {
            result += (current_region_size as i64) * fences_sides(&current_region_fences);
            current_region_size = 0;
            current_region_fences = Vec::new();
            to_visit_current_region.push(to_visit_in_new_region);
        } else {
            result += (current_region_size as i64) * fences_sides(&current_region_fences);
            break;
        }
    }

    result
}

fn neighbours(position: &Position) -> Vec<Position> {
    vec![
        Position {
            row: position.row - 1,
            column: position.column,
        },
        Position {
            row: position.row,
            column: position.column - 1,
        },
        Position {
            row: position.row + 1,
            column: position.column,
        },
        Position {
            row: position.row,
            column: position.column + 1,
        },
    ]
}

fn neighbour_value(map: &[Vec<char>], position: &Position) -> Option<char> {
    if position.row >= 0 && position.column >= 0 {
        map.get(position.row as usize).and_then(|row| row.get(position.column as usize)).cloned()
    } else {
        None
    }
}

fn fences_sides(fences: &[(Position, Position)]) -> i64 {
    fences.len() as i64
}


#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: i64,
    column: i64,
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
