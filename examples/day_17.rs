use itertools::{Itertools};
use sscanf::sscanf;
use std::fs::read_to_string;

fn main() {
    println!("Hello day 17!");
    let input = read_to_string("inputs/day_17/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> String {
    let mut input_lines = input.split("\n");

    let register_a = sscanf!(input_lines.next().unwrap(), "Register A: {i64}").unwrap();
    let register_b = sscanf!(input_lines.next().unwrap(), "Register B: {i64}").unwrap();
    let register_c = sscanf!(input_lines.next().unwrap(), "Register C: {i64}").unwrap();
    input_lines.next();
    let program_str = sscanf!(input_lines.next().unwrap(), "Program: {String}").unwrap();

    let instructions: Vec<u8> = program_str.split(",").map(|v| v.parse().unwrap()).collect();

    let mut program = Program {
        register_a,
        register_b,
        register_c,
        instructions,
    };

    let mut current_instruction_index = 0;
    let mut output_values: Vec<i64> = vec![];

    while current_instruction_index < program.instructions.len() {
        let operand = program.instructions[current_instruction_index+1];
        let instruction = program.instructions[current_instruction_index];
        match instruction {
            0 => {
                program.register_a = program.register_a / 2i64.pow(program.combo(operand) as u32)
            },
            1 => {
                program.register_b = program.register_b ^ operand as i64
            },
            2 => {
                program.register_b = program.combo(operand) % 8;
            },
            3 => {
                if program.register_a != 0 {
                    current_instruction_index = operand as usize;
                    continue
                }
            },
            4 => {
                program.register_b = program.register_b ^ program.register_c
            },
            5 => {
                output_values.push(program.combo(operand) % 8)
            },
            6 => {
                program.register_b = program.register_a / 2i64.pow(program.combo(operand) as u32)
            },
            7 => {
                program.register_c = program.register_a / 2i64.pow(program.combo(operand) as u32)
            },
            _ => panic!("Unknown instruction"),
        }

        current_instruction_index += 2;
    }

    output_values.into_iter().join(",")
}

struct Program {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    instructions: Vec<u8>,
}

impl Program {
    fn combo(&self, value: u8) -> i64 {
        match value {
            0 | 1 | 2| 3 => value as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Unknown combo operand {value}")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "Register A: 729\n\
Register B: 0\n\
Register C: 0\n\
\n\
Program: 0,1,5,4,3,0";
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
