use itertools::{any, Itertools};
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
    let mut current_region_fences : Vec<FencePosition> = Vec::new();
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
                        current_region_fences.push(FencePosition{
                            group_side: position_in_current_region,
                            outer_side: neighbour,
                        })
                    }

                } else {
                    current_region_fences.push(FencePosition{
                        group_side: position_in_current_region,
                        outer_side: neighbour,
                    })
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

fn fences_sides(fences: &[FencePosition]) -> i64 {
    if fences.is_empty() {
        return 0
    }
    let mut fences_left = fences.into_iter().cloned().collect_vec();
    let mut fences_groups = vec![vec![fences_left.pop().unwrap()]];


    while !fences_left.is_empty() {

        let current_group = fences_groups.last_mut().unwrap();

        let fences_left_matching_to_group_position = fences_left.iter().position(|fence| current_group.iter().any(|group_fence| fence_matching(group_fence, fence)));

        if let Some(matching_position) = fences_left_matching_to_group_position {
            current_group.push(fences_left.remove(matching_position))
        } else {
            fences_groups.push(vec![fences_left.pop().unwrap()])
        }
    }

    fences_groups.len() as i64
}

fn fence_matching(f1: &FencePosition, f2: &FencePosition) -> bool{
    let f1_row_diff = f1.group_side.row - f1.outer_side.row;
    let f1_column_diff = f1.group_side.column - f1.outer_side.column;

    let f2_row_diff = f2.group_side.row - f2.outer_side.row;
    let f2_column_diff = f2.group_side.column - f2.outer_side.column;


    f1_row_diff == f2_row_diff && f1_column_diff == f2_column_diff
        && neighbours(&f1.outer_side).contains(&f2.outer_side)
        && neighbours(&f1.group_side).contains(&f2.group_side)
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: i64,
    column: i64,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct FencePosition {
    group_side: Position,
    outer_side: Position
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
        assert_eq!(solve(input), 1206);
    }
}
