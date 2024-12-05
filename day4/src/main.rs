use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut chars: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        chars.push(line.chars().collect());
    }
    part_1(&chars);

    part_2(&chars);
}

fn part_1(chars: &[Vec<char>]) {
    let mut counter = 0;

    for line in 0..chars.len() {
        for col in 0..chars[0].len() {
            if chars[line][col] == 'X' {
                //right
                let l = &chars[line];
                if col + 3 < chars[0].len()
                    && l[col + 1] == 'M'
                    && l[col + 2] == 'A'
                    && l[col + 3] == 'S'
                {
                    counter += 1;
                }

                // left (backwards)
                if col >= 3 && l[col - 1] == 'M' && l[col - 2] == 'A' && l[col - 3] == 'S' {
                    counter += 1;
                }

                // down
                if line + 3 < chars.len()
                    && chars[line + 1][col] == 'M'
                    && chars[line + 2][col] == 'A'
                    && chars[line + 3][col] == 'S'
                {
                    counter += 1;
                }

                // up
                if line >= 3
                    && chars[line - 1][col] == 'M'
                    && chars[line - 2][col] == 'A'
                    && chars[line - 3][col] == 'S'
                {
                    counter += 1;
                }

                // diagonal bottom right
                if line + 3 < chars.len()
                    && col + 3 < chars[0].len()
                    && chars[line + 1][col + 1] == 'M'
                    && chars[line + 2][col + 2] == 'A'
                    && chars[line + 3][col + 3] == 'S'
                {
                    counter += 1;
                }

                // diagonal bottom left
                if line + 3 < chars.len()
                    && col >= 3
                    && chars[line + 1][col - 1] == 'M'
                    && chars[line + 2][col - 2] == 'A'
                    && chars[line + 3][col - 3] == 'S'
                {
                    counter += 1;
                }

                // diagonal top left
                if line >= 3
                    && col >= 3
                    && chars[line - 1][col - 1] == 'M'
                    && chars[line - 2][col - 2] == 'A'
                    && chars[line - 3][col - 3] == 'S'
                {
                    counter += 1;
                }

                // diagonal top right
                if line >= 3
                    && col + 3 < chars[0].len()
                    && chars[line - 1][col + 1] == 'M'
                    && chars[line - 2][col + 2] == 'A'
                    && chars[line - 3][col + 3] == 'S'
                {
                    counter += 1;
                }
            }
        }
    }

    println!("counter: {counter}");
}

fn part_2(chars: &[Vec<char>]) {
    let mut counter = 0;

    let mut positions = HashSet::new();

    for line in 0..chars.len() {
        for col in 0..chars[0].len() {
            let old_counter = counter;
            if chars[line][col] == 'A'
                && line > 0
                && line + 1 < chars.len()
                && col > 0
                && col + 1 < chars[0].len()
            {
                /*
                   M S
                   M S
                */
                if chars[line - 1][col - 1] == 'M'
                    && chars[line - 1][col + 1] == 'S'
                    && chars[line + 1][col - 1] == 'M'
                    && chars[line + 1][col + 1] == 'S'
                {
                    counter += 1;
                }

                /*
                   M M
                   S S
                */
                if chars[line - 1][col - 1] == 'M'
                    && chars[line - 1][col + 1] == 'M'
                    && chars[line + 1][col - 1] == 'S'
                    && chars[line + 1][col + 1] == 'S'
                {
                    counter += 1;
                }

                /*
                   S M
                   S M
                */
                if chars[line - 1][col - 1] == 'S'
                    && chars[line - 1][col + 1] == 'M'
                    && chars[line + 1][col - 1] == 'S'
                    && chars[line + 1][col + 1] == 'M'
                {
                    counter += 1;
                }

                /*
                   S S
                   M M
                */
                if chars[line - 1][col - 1] == 'S'
                    && chars[line - 1][col + 1] == 'S'
                    && chars[line + 1][col - 1] == 'M'
                    && chars[line + 1][col + 1] == 'M'
                {
                    counter += 1;
                }
            }
            if counter > old_counter {
                positions.insert((line, col));
            }
        }
    }

    println!("counter2: {counter}");
}
