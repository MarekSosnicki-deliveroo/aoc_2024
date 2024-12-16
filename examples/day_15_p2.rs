use itertools::Itertools;
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
    let mut map = parts
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => ['#', '#'],
                    '.' => ['.', '.'],
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => panic!("Unknown char in map {c}"),
                })
                .collect_vec()
        })
        .collect_vec();
    let moves = parts
        .next()
        .unwrap()
        .lines()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect_vec();

    let mut robot_position = find_start(&map);
    map[robot_position.row][robot_position.column] = '.';


    for move_direction in moves {
        // map[robot_position.row][robot_position.column] = '@';
        // print_map(&map);
        // map[robot_position.row][robot_position.column] = '.';

        let new_position = try_move(&map, robot_position, move_direction).unwrap();

        let new_position_value = map[new_position.row][new_position.column];
        match new_position_value {
            '#' => {}
            '.' => {
                robot_position = new_position;
            }
            '[' | ']' => {
                let (left, right) = box_coordinates(&map, new_position);

                let mut all_box_coordinates = vec![(left, '['), (right, ']')];

                let mut next_positions = vec![
                    try_move(&map, left, move_direction).unwrap(),
                    try_move(&map, right, move_direction).unwrap(),
                ];

                let mut should_move = true;

                loop{
                    // At least one wall - move blocked
                    if next_positions.iter().any(|p| map[p.row][p.column] == '#') {
                        should_move = false;
                        break;
                    }
                    // All empty start moving
                    if next_positions.iter().all(|p| map[p.row][p.column] == '.') {
                        break;
                    }

                    next_positions = next_positions.into_iter().flat_map(|p| {
                        let p_value = map[p.row][p.column];
                        if p_value == '[' || p_value == ']' {
                            let (l, r) = box_coordinates(&map, p);
                            all_box_coordinates.push((l, '['));
                            all_box_coordinates.push((r, ']'));

                            let l_moved = try_move(&map, l, move_direction).unwrap();
                            let r_moved = try_move(&map, r, move_direction).unwrap();

                            if l_moved == r {
                                vec![r_moved]
                            } else if r_moved == l {
                                vec![l_moved]
                            } else {
                                vec![l_moved, r_moved]
                            }
                        } else {
                            vec![]
                        }
                    }).collect()
                }


                if should_move {
                    robot_position = new_position;

                    for (b_pos,_) in all_box_coordinates.iter() {
                        map[b_pos.row][b_pos.column] = '.';
                    }
                    for (b_pos, value) in all_box_coordinates.iter() {
                        let b_new_pos = try_move(&map, *b_pos, move_direction).unwrap();
                        map[b_new_pos.row][b_new_pos.column] = *value;
                    }
                }
            }
            _ => panic!("Unknwn new position value {new_position_value}"),
        }
    }

    let mut result = 0;
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] == '[' {
                result += 100 * row as i64 + column as i64;
            }
        }
    }
    result
}

fn box_coordinates(map: &[Vec<char>], one_of_edges: Position) -> (Position, Position) {
    let value = map[one_of_edges.row][one_of_edges.column];
    if value == ']' {
        (
            Position {
                row: one_of_edges.row,
                column: one_of_edges.column - 1,
            },
            one_of_edges,
        )
    } else if value == '[' {
        (
            one_of_edges,
            Position {
                row: one_of_edges.row,
                column: one_of_edges.column + 1,
            },
        )
    } else {
        panic!("Unknow character not a box")
    }
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

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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
            _ => panic!("Failed to get direction from sign {value}"),
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

fn print_map(map: &[Vec<char>]) {
    println!("MAP!!");
    println!("{}", map.iter().map(|row| row.iter().join("")).join("\n"));
    println!("----");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let input = "#######\n\
#...#.#\n\
#.....#\n\
#..OO@#\n\
#..O..#\n\
#.....#\n\
#######\n\
\n\
<vv<<^^<<^^";
        solve(input);
    }

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
        assert_eq!(solve(input), 9021);
    }
}
