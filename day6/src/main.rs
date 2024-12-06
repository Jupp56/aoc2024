use std::{collections::HashSet, fs::File, io::Read};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut parsed = Vec::new();

    for line in input.lines() {
        let mut v = Vec::new();
        for col in line.chars() {
            v.push(col);
        }
        parsed.push(v);
    }

    let mut start_row = 0;
    let mut start_col = 0;

    'outer: for (row_index, row_str) in parsed.iter().enumerate() {
        for (col_index, char) in row_str.iter().enumerate() {
            if *char == '^' {
                start_row = row_index;
                start_col = col_index;
                break 'outer;
            }
        }
    }

    part_1(start_row,start_col, &parsed);
    part_2(start_row,start_col, &parsed);
}

fn part_1(mut row: usize, mut col: usize, parsed: &[Vec<char>]) {
    let mut dir = Direction::Up;

    let mut positions = HashSet::new();

    loop {
        positions.insert((row, col));
        match dir {
            Direction::Up => {
                if row == 0 {
                    break;
                }

                if parsed[row - 1][col] == '#' {
                    dir = Direction::Right;
                } else {
                    row -= 1;
                }
            }
            Direction::Down => {
                if row == parsed.len() - 1 {
                    break;
                }

                if parsed[row + 1][col] == '#' {
                    dir = Direction::Left;
                } else {
                    row += 1;
                }
            }
            Direction::Left => {
                if col == 0 {
                    break;
                }

                if parsed[row][col - 1] == '#' {
                    dir = Direction::Up;
                } else {
                    col -= 1;
                }
            }
            Direction::Right => {
                if col == parsed[row].len() - 1 {
                    break;
                }
                if parsed[row][col + 1] == '#' {
                    dir = Direction::Down;
                } else {
                    col += 1;
                }
            }
        }
    }

    println!("part1: {}", positions.len());
}

fn part_2(start_row: usize, start_col: usize, parsed: &[Vec<char>]) {
    let l = parsed[0].len();
    let mut counter = 0;
    for i in 0..parsed.len() {
        for j in 0..l {
            if parsed[i][j] == '.' {
                let mut cl = parsed.to_owned();

                cl[i][j] = '#';

                if check_part_2(start_row, start_col, &cl) {
                    counter += 1;
                }
            }
        }
    }

    println!("Part 2: {counter}");
}

fn check_part_2(mut row: usize, mut col: usize, parsed: &[Vec<char>]) -> bool {
    let mut dir = Direction::Up;

    let mut positions = HashSet::new();

    loop {
        if !positions.insert((row, col, dir)) {
            break true;
        }
        match dir {
            Direction::Up => {
                if row == 0 {
                    break false;
                }

                if parsed[row - 1][col] == '#' {
                    dir = Direction::Right;
                } else {
                    row -= 1;
                }
            }
            Direction::Down => {
                if row == parsed.len() - 1 {
                    break false;
                }

                if parsed[row + 1][col] == '#' {
                    dir = Direction::Left;
                } else {
                    row += 1;
                }
            }
            Direction::Left => {
                if col == 0 {
                    break false;
                }

                if parsed[row][col - 1] == '#' {
                    dir = Direction::Up;
                } else {
                    col -= 1;
                }
            }
            Direction::Right => {
                if col == parsed[row].len() - 1 {
                    break false;
                }
                if parsed[row][col + 1] == '#' {
                    dir = Direction::Down;
                } else {
                    col += 1;
                }
            }
        }
    }
}
