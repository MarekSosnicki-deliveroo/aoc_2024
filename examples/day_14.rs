extern crate core;

use boolinator::Boolinator;
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

const DURATION: i64 = 100;

fn main() {
    println!("Hello day 14!");
    let input = read_to_string("inputs/day_14/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim(), 103, 101),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str, rows: usize, columns: usize) -> i64 {
    let end_positions: Vec<Position> = input
        .lines()
        .map(|line| sscanf!(line, "p={i64},{i64} v={i64},{i64}").unwrap())
        .map(|(start_column, start_row, column_velocity, row_velocity)| {
            let end_row_raw = (start_row + row_velocity * DURATION) % (rows as i64);
            let end_row = if end_row_raw < 0 {
                end_row_raw + (rows as i64)
            } else {
                end_row_raw
            };

            let end_column_raw = (start_column + column_velocity * DURATION) % (columns as i64);
            let end_column = if end_column_raw < 0 {
                end_column_raw + (columns as i64)
            } else {
                end_column_raw
            };


            Position {
                row: end_row.try_into().unwrap(),
                column: end_column.try_into().unwrap(),
            }
        })
        .collect();

    print_map(rows, columns, &end_positions);

    let quadrants = vec![
        (0..(rows / 2), 0..(columns / 2)),
        (0..(rows / 2), (columns / 2 + 1)..columns),
        ((rows / 2 + 1)..rows, 0..(columns / 2)),
        ((rows / 2 + 1)..rows, (columns / 2 + 1)..columns),
    ];

    quadrants
        .iter()
        .map(|(row_range, column_range)| {
            let res = end_positions
                .iter()
                .filter(|p| row_range.contains(&p.row) && column_range.contains(&p.column))
                .count() as i64;
            println!("Quadrant has {res} robots");
            res
        })
        .product()
}

fn print_map(rows: usize, columns: usize, positions: &[Position]) {
    let mut map = vec![vec![0; columns];rows];
    for p in positions {
        map[p.row][p.column] += 1;
    }
    for row in map.iter() {
        println!();
        for c in row.iter() {
            if *c == 0 {
                print!(".")
            } else {
                print!("{c}")
            }
        }
    }
    println!();


}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_simple() {
    //     let input = "p=2,4 v=2,-3";
    //     assert_eq!(12, solve(input, 7, 11))
    // }

    #[test]
    fn test_solve() {
        let input = "p=0,4 v=3,-3\n\
p=6,3 v=-1,-3\n\
p=10,3 v=-1,2\n\
p=2,0 v=2,-1\n\
p=0,0 v=1,3\n\
p=3,0 v=-2,-2\n\
p=7,6 v=-1,-3\n\
p=3,0 v=-1,-2\n\
p=9,3 v=2,3\n\
p=7,3 v=-1,2\n\
p=2,4 v=2,-3\n\
p=9,5 v=-3,-3";
        assert_eq!(12, solve(input, 7, 11))
    }
}
