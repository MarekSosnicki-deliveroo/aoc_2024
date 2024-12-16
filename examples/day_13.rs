use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;

// For Part 1 OFFSET = 0
// const OFFSET: i64 = 0;
// For Part 2 OFFSET = 10000000000000
const OFFSET: i64 = 10000000000000;

fn main() {
    println!("Hello day 13!");
    let input = read_to_string("inputs/day_13/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    input
        .split("\n\n")
        .map(|case_str| {
            let mut lines = case_str.split("\n");

            let (button_a_x, button_a_y) =
                sscanf!(lines.next().unwrap(), "Button A: X+{i64}, Y+{i64}").unwrap();

            let (button_b_x, button_b_y) =
                sscanf!(lines.next().unwrap(), "Button B: X+{i64}, Y+{i64}").unwrap();

            let (price_x, price_y) =
                sscanf!(lines.next().unwrap(), "Prize: X={i64}, Y={i64}").unwrap();

            solve_inner(
                button_a_x, button_a_y, button_b_x, button_b_y, price_x + OFFSET, price_y + OFFSET,
            )
        })
        .collect_vec()
        .into_iter()
        .inspect(|v| println!("Solution {}", v))
        .sum()
}

fn solve_inner(
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
    price_x: i64,
    price_y: i64,
) -> i64 {
    let b_count = (price_x * button_a_y - price_y * button_a_x)/(button_b_x * button_a_y - button_b_y * button_a_x);
    let a_count = (price_x - b_count * button_b_x)/ button_a_x;

    if price_y == a_count * button_a_y + b_count * button_b_y  && price_x == a_count * button_a_x + b_count * button_b_x {
        a_count * 3 + b_count
    } else {
        0
    }


    // Below there is a Solver approach, but it failed for the part2

    // let mut vars = variables!();
    //
    // let a_count_var = vars.add(variable().integer().name("a_count"));
    // let b_count_var = vars.add(variable().integer().name("b_count"));
    // let cost_var = vars.add(variable().integer().name("cost"));
    //
    // let solver = vars
    //     .minimise(a_count_var)
    //     .using(default_solver)
    //     .with(constraint!(
    //         button_a_x as f64 * a_count_var + button_b_x as f64 * b_count_var
    //             ==  price_x as f64
    //     ))
    //     .with(constraint!(
    //         button_a_y as f64 * a_count_var + button_b_y as f64 * b_count_var
    //             ==  price_y as f64
    //     ))
    //     .with(constraint!(3 * a_count_var + b_count_var == cost_var));
    //
    // if let Ok(solution) = solver.solve() {
    //
    //     println!("A Count Value {}", solution.value(a_count_var));
    //     println!("B Count Value {}", solution.value(b_count_var));
    //     println!("Solution value is {}", solution.value(cost_var));
    //
    //     let x_value = button_a_x as f64 * solution.value(a_count_var) + button_b_x as f64 * solution.value(b_count_var);
    //     let x_expected =  price_x as f64;
    //     println!("Check x {} == {} => {}", x_value as i64, x_expected as i64, x_value as i64 == x_expected as i64);
    //     assert_eq!(x_value as i64, x_expected as i64);
    //
    //     let y_value = button_a_y as f64 * solution.value(a_count_var) + button_b_y as f64 * solution.value(b_count_var);
    //     let y_expected =  price_y as f64;
    //     println!("Check y {} == {} => {}", y_value as i64 , y_expected as i64 , y_value as i64 == y_expected as i64);
    //     assert_eq!(y_value as i64, y_expected as i64);
    //
    //     solution.value(cost_var).round() as i64
    // } else {
    //     println!("Failed to find solution",);
    //     0
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Button A: X+94, Y+34\n\
Button B: X+22, Y+67\n\
Prize: X=8400, Y=5400\n\
\n\
Button A: X+26, Y+66\n\
Button B: X+67, Y+21\n\
Prize: X=12748, Y=12176\n\
\n\
Button A: X+17, Y+86\n\
Button B: X+84, Y+37\n\
Prize: X=7870, Y=6450\n\
\n\
Button A: X+69, Y+23\n\
Button B: X+27, Y+71\n\
Prize: X=18641, Y=10279";
        if OFFSET == 0 {
            assert_eq!(solve(input), 480);
        } else {
            assert_eq!(solve(input), 875318608908);
        }
    }
}
