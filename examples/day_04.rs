use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 4!");
    let input = read_to_string("inputs/day_04/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    let input_lines: Vec<String> = input.lines().map(|line| line.to_string()).collect();

    let input_columns: Vec<String> = (0..input_lines[0].len())
        .map(|c| {
            (0..input_lines.len())
                .map(|r| input_lines[r].get(c..(c + 1)).unwrap())
                .join("")
        })
        .collect();

    println!("Columns are {:?}", input_columns);

    let diagonals_left_right : Vec<String> = (0..input_lines.len())
        .map(
            |r_start| {
                (0..input_lines[0].len())
                    .filter_map(|c| input_lines.get(r_start + c)?.get(c..c + 1))
                    .join("")
            })
        .chain(
            (1..input_lines[0].len()).map(|c_start| {
                (0..input_lines.len())
                    .filter_map(|r| input_lines.get(r)?.get((c_start+r)..(c_start+r + 1)))
                    .join("")
            })
        )


        .collect();

    println!("Diagonals left right are {:?}", diagonals_left_right);


    let diagonals_right_left : Vec<String> = (0..input_lines.len())
        .map(
            |r_start| {
                (0..input_lines[0].len())
                    .filter_map(|c| input_lines.get(r_start + c)?.get((input_lines[0].len() -1 - c)..(input_lines[0].len() - c)))
                    .join("")
            })
        .chain(
            (0..(input_lines[0].len() - 1)).rev().map(|c_start| {
                (0..(input_lines.len().min(c_start + 1)))
                    .filter_map(|r| input_lines.get(r)?.get((c_start-r)..(c_start-r +1)))
                    .join("")
            })
        )


        .collect();

    println!("Diagonals right left are {:?}", diagonals_right_left);


    calculate_xmas(&input_lines) + calculate_xmas(&input_columns) + calculate_xmas(&diagonals_left_right)  + calculate_xmas(&diagonals_right_left)
}

fn calculate_xmas(input: &[String]) -> i64 {
    input
        .iter()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>() as i64
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "MMMSXXMASM\n\
MSAMXMSMSA\n\
AMXSXMAAMM\n\
MSAMASMSMX\n\
XMASAMXAMM\n\
XXAMMXXAMA\n\
SMSMSASXSS\n\
SAXAMASAAA\n\
MAMMMXMMMM\n\
MXMXAXMASX";
        assert_eq!(solve(input), 18);
    }
}
