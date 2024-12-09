use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 7!");
    let input = read_to_string("inputs/day_07/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    input.lines().map(|line|
        {
            let (target, values_str) = sscanf!(line, "{i64}: {String}").unwrap();

            let values : Vec<i64> = values_str.split(" ").map(|value| value.parse().unwrap()).collect();

            (target, values)
        }
    ).filter(|(target, values)| can_be_made_valid(*target, &values))
        .map(|(target, _)| target)
        .sum()
}

fn can_be_made_valid(target: i64, values: &[i64]) -> bool {
    check_valid_recursive(target, values[0], &values[1..])
}

fn check_valid_recursive(target: i64, current: i64, remaining: &[i64]) -> bool {
    if current > target {
        return false
    }
    if remaining.len() == 0 {
        target == current
    } else {
        check_valid_recursive(target, current + remaining[0], &remaining[1..]) ||
            check_valid_recursive(target, current * remaining[0], &remaining[1..]) ||
        check_valid_recursive(target, format!("{}{}", current, remaining[0]).parse::<i64>().unwrap(), &remaining[1..])

    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "190: 10 19\n\
3267: 81 40 27\n\
83: 17 5\n\
156: 15 6\n\
7290: 6 8 6 15\n\
161011: 16 10 13\n\
192: 17 8 14\n\
21037: 9 7 18 13\n\
292: 11 6 16 20";
        assert_eq!(solve(input), 11387);
    }
}
