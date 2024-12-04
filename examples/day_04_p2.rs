use std::fs::read_to_string;

fn main() {
    println!("Hello day 4!");
    let input = read_to_string("inputs/day_04/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    let input_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let input_lines_ref = &input_lines;

    let col_len = input_lines.len();

    (1..(col_len - 1))
        .flat_map(|row| {
            (1..(col_len - 1)).map(move |column| {
                input_lines_ref[row][column] == 'A'
                    && are_mas(
                        input_lines_ref[row + 1][column + 1],
                        input_lines_ref[row - 1][column - 1],
                    )
                    && are_mas(
                        input_lines_ref[row + 1][column - 1],
                        input_lines_ref[row - 1][column + 1],
                    )
            })
        })
        .filter(|v| *v)
        .count() as i64
}

fn are_mas(c1: char, c2: char) -> bool {
    (c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "M.S\n\
.A.\n\
M.S";
        assert_eq!(solve(input), 1);
    }

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
        assert_eq!(solve(input), 9);
    }
}
