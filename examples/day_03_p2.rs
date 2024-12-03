use sscanf::sscanf;
use std::fs::read_to_string;
use itertools::Itertools;

fn main() {
    println!("Hello day 3!");
    let input = read_to_string("inputs/day_03/input").unwrap();

    println!("Result is {}", solve(input.trim()))
}

fn solve(input: &str) -> i64 {
    let mut string_to_analyse = "xx".to_string() +  &input.replace("\n","W") + "xx";

    let mut result = 0;
    let mut enabled = true;
    while let Ok((before, v1, v2, reminder)) = sscanf!(string_to_analyse, "{str}mul({i64},{i64}){str}" ) {
        if let Some((_, last_change)) = before.match_indices("do()").chain(before.match_indices("don't()")).sorted_by_key(|(position, _)| *position).last() {
            if last_change == "do()" {
                enabled = true
            } else {
                enabled = false
            }
        }

        if enabled {
            result += v1 * v2;
        }

        string_to_analyse = "xx".to_string()  + reminder;
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve(input), 48);
    }
}
