use boolinator::Boolinator;
use good_lp::{constraint, default_solver, variable, variables, Solution, SolverModel};
use itertools::Itertools;
use sscanf::sscanf;
use std::fs::read_to_string;


fn main() {
    println!("Hello day 15!");
    let input = read_to_string("inputs/day_15/input").unwrap();

    let start_solve = std::time::Instant::now();
    println!(
        "Solve result is {} time {}ms",
        solve(input.trim()),
        start_solve.elapsed().as_millis()
    );
}

fn solve(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let mut map = parts.next().unwrap().split("\n").map(|line| line.chars().collect_vec()).collect_vec();
    let moves = parts.next().unwrap().lines().flat_map(|line| line.chars()).map(Direction::from).collect_vec();

    let mut robot_position = find_start(&map);
    map[robot_position.row][robot_position.column] = '.';

    for move_direction in moves {
        // print_map(&map);

        let new_position = try_move(&map, robot_position, move_direction).unwrap();

        let new_position_value = map[new_position.row][new_position.column];
        match new_position_value {
            '#' => {},
            '.' => {
                robot_position = new_position;
            },
            'O' => {
                let mut wall_or_empty_after_blocks = new_position;
                while true {
                    wall_or_empty_after_blocks = try_move(&map, wall_or_empty_after_blocks, move_direction).unwrap();
                    if map[wall_or_empty_after_blocks.row][wall_or_empty_after_blocks.column] != 'O' {
                        break;
                    }
                }

                if map[wall_or_empty_after_blocks.row][wall_or_empty_after_blocks.column] == '.' {
                    robot_position = new_position;
                    map[new_position.row][new_position.column] = '.';
                    map[wall_or_empty_after_blocks.row][wall_or_empty_after_blocks.column] = 'O';
                }
            }
            _ => panic!("Unknwn new position value {new_position_value}")
        }
    }


    let mut result = 0;
    for row in 0..map.len() {
        for column in 0..map[0].len() {
          if map[row][column] == 'O' {
              result += 100 * row as i64 + column as i64;
          }
        }
    }
    result
}

fn find_start(map: &[Vec<char>]) -> Position {
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if map[row][column] == '@' {
                return Position { row, column };
            }
        }
    }
    panic!("No starting position for robot found")
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Failed to get direction from sign {value}")
        }
    }
}

fn try_move(map: &[Vec<char>], start: Position, direction: Direction) -> Option<Position> {
    match direction {
        Direction::Up => {
            if start.row == 0 {
                None
            } else {
                Some(Position {
                    row: start.row - 1,
                    column: start.column,
                })
            }
        }
        Direction::Right => {
            if start.column + 1 == map[start.row].len() {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column + 1,
                })
            }
        }
        Direction::Down => {
            if start.row + 1 == map.len() {
                None
            } else {
                Some(Position {
                    row: start.row + 1,
                    column: start.column,
                })
            }
        }
        Direction::Left => {
            if start.column == 0 {
                None
            } else {
                Some(Position {
                    row: start.row,
                    column: start.column - 1,
                })
            }
        }
    }
}

fn print_map(map: &[Vec<char>])  {
    println!("MAP!!");
    println!("{}", map.iter().map(|row| row.iter().join("")).join("\n"));
    println!("----");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "##########\n\
#..O..O.O#\n\
#......O.#\n\
#.OO..O.O#\n\
#..O@..O.#\n\
#O#..O...#\n\
#O..O..O.#\n\
#.OO.O.OO#\n\
#....O...#\n\
##########\n\
\n\
<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!(solve(input), 10092);
    }
}
