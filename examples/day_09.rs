use boolinator::Boolinator;
use itertools::{Itertools, Position};
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 9!");
    let input = read_to_string("inputs/day_09/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let input_values: Vec<usize> = input.chars().map(|c| (c as u8 - '0' as u8) as usize).collect();
    println!("Input values {:?}", input_values);
    let mut forward_input_iter_index = 0;
    let mut backward_input_iter_index = input.len() - 1;
    let mut checksum = 0;
    let mut current_checksum_index  = 0i64;

    let mut current_forward_id = 0i64;
    let mut current_back_id = (input.len() / 2) as i64;

    let mut forward_blank_spaces_left = 0;
    let mut backward_values_left = input_values[backward_input_iter_index];

    assert!(input.len() % 2 == 1);

    while forward_input_iter_index < backward_input_iter_index{
        // println!("---------");
        // println!("forward_input_iter_index: {forward_input_iter_index}");
        // println!("backward_input_iter_index: {backward_input_iter_index}");
        // println!("current_checksum_index {current_checksum_index}");
        // println!("current_forward_id {current_forward_id}");
        // println!("current_back_id {current_back_id}");
        // println!("forward_blank_spaces_left {forward_blank_spaces_left}");
        // println!("backward_values_left {backward_values_left}");
        // println!("checksum {checksum}");

        if forward_input_iter_index % 2 == 0 {
            for _ in 0..input_values[forward_input_iter_index] {
                checksum += current_checksum_index * current_forward_id;
                current_checksum_index+=1;
            }
            current_forward_id +=1;
            forward_input_iter_index += 1;
            forward_blank_spaces_left = input_values[forward_input_iter_index];
        } else {
            if forward_blank_spaces_left == 0 {
                forward_input_iter_index += 1;
                continue;
            }

            if backward_values_left == 0 {
                backward_input_iter_index -=2;
                current_back_id -=1;
                backward_values_left = input_values[backward_input_iter_index];
                continue;
            }

            checksum += current_back_id * current_checksum_index;
            forward_blank_spaces_left-=1;
            backward_values_left-=1;
            current_checksum_index+=1;
        }
    }

    for _ in 0..backward_values_left {
        checksum += current_back_id * current_checksum_index;
        current_checksum_index += 1;
    }




    checksum

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "12345";
        assert_eq!(solve(input), 2+2*2+3+4+5+6*2+7*2+8*2);
    }
    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928);
    }
}
