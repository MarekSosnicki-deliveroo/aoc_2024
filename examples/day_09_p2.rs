use boolinator::Boolinator;
use itertools::{Itertools, Position};
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

fn main() {
    println!("Hello day 8!");
    let input = read_to_string("inputs/day_09/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

#[derive(Debug, Clone)]
enum MemoryBlock {
    Occupied { len: i64, file_id: i64 },
    Free { len: i64 },
}

impl MemoryBlock {
    fn file_id(&self) -> Option<i64> {
        match self {
            MemoryBlock::Occupied { file_id, .. } => Some(*file_id),
            MemoryBlock::Free { .. } => None,
        }
    }
    fn len(&self) -> i64 {
        match self {
            MemoryBlock::Occupied { len, .. } => *len,
            MemoryBlock::Free { len } => *len,
        }
    }
}

fn solve(input: &str) -> i64 {
    let input_values: Vec<i64> = input
        .chars()
        .map(|c| (c as u8 - '0' as u8) as i64)
        .collect();
    println!("Input values {:?}", input_values);

    let mut memory_map = input_values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if index % 2 == 0 {
                MemoryBlock::Occupied {
                    len: *value,
                    file_id: (index / 2) as i64,
                }
            } else {
                MemoryBlock::Free { len: *value }
            }
        })
        .collect_vec();

    for id_to_move in (0..((input_values.len() / 2 + 1) as i64)).rev() {
        // print_map(&mut memory_map);
        let current_position_of_the_block = memory_map
            .iter()
            .position(|memory_block| memory_block.file_id() == Some(id_to_move))
            .unwrap();

        let block_len = memory_map[current_position_of_the_block].len();

        let found_free_space_position = memory_map.iter().take(current_position_of_the_block).position(|memory_block| {
            memory_block.file_id() == None && memory_block.len() >= block_len
        });

        if let Some(free_position) = found_free_space_position {
            let free_position_len = memory_map[free_position].len();
            println!("Id moved to {free_position}, free_position len {free_position_len}");

            memory_map[free_position] = memory_map[current_position_of_the_block].clone();
            memory_map[current_position_of_the_block] = MemoryBlock::Free {len: block_len};
            if free_position_len > block_len {
                memory_map.insert(
                    free_position + 1,
                    MemoryBlock::Free {
                        len: free_position_len - block_len,
                    },
                )
            }
        }
    }

    // println!("Result memory map {memory_map:?}");
    print_map(&mut memory_map);


    memory_map
        .iter()
        .fold(
            (0i64, 0i64),
            |(start_index, checksum), current| match current {
                MemoryBlock::Occupied { len, file_id } => (
                    start_index + len,
                    checksum + (0..*len).map(|i| (i + start_index) * file_id).sum::<i64>(),
                ),
                MemoryBlock::Free { len } => (start_index + len, checksum),
            },
        )
        .1
}

fn print_map(memory_map: &mut Vec<MemoryBlock>) {
    println!("MAP:");
    for m in memory_map.iter() {
        match m {
            MemoryBlock::Occupied { len, file_id } => {
                for _ in 0..*len {
                    print!("{file_id}")
                }
            }
            MemoryBlock::Free { len } => {
                for _ in 0..*len {
                    print!(".")
                }
            }
        }
    }
    println!("\nMAP end");
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 2858);
    }
}
