use std::fs::read_to_string;

fn main() {
    println!("Hello day 22!");
    let input = read_to_string("inputs/day_22/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    input.split("\n").map(|v| {
        let start_number :i64 = v.parse().unwrap();
        let secret_number = calculate_secret(start_number);
        println!("- {start_number}: {secret_number}");
        secret_number
    }).sum()
}

fn calculate_secret(start_number: i64) -> i64 {
    const NO_ITERATIONS: usize = 2000;
    let mut number = start_number;
    for _ in 0..NO_ITERATIONS {
        number = (number*64) ^ number;
        number %= 16777216;
        number =  (number/  32) ^ number;
        number %= 16777216;
        number = (number*2048) ^ number;
        number %= 16777216;
    }
    number
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_solve() {
        let input = "1\n\
10\n\
100\n\
2024";
        assert_eq!(solve(input), 37327623);
    }
}
