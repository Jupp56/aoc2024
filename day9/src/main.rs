use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut file_space = Vec::new();

    let input: Vec<usize> = input.chars().map(|c| c as usize - 48).collect();

    for (index, number) in input.iter().enumerate() {
        if index % 2 == 0 {
            for _ in 0..*number {
                file_space.push((index / 2).to_string());
            }
        } else {
            for _ in 0..*number {
                file_space.push(".".to_string());
            }
        }
    }

    part_1(file_space.clone());

    part_2(file_space);
}

fn part_1(mut file_space: Vec<String>) {
    for i in 0..file_space.len() {
        let rindex = file_space.len() - 1 - i;

        if file_space[rindex] != "." {
            for j in 0..rindex {
                if file_space[j] == "." {
                    file_space[j] = file_space[rindex].clone();
                    file_space[rindex] = ".".to_string();
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", calculate_checksum(&file_space));
}

fn part_2(mut file_space: Vec<String>) {
    let mut current_symbol_end = file_space.len() - 1;
    let mut current_symbol = "start".to_string();

    for r_index in (0..file_space.len()).rev() {
        if file_space[r_index] != "." {
            if file_space[r_index] != current_symbol {
                current_symbol_end = r_index;
            }

            current_symbol = file_space[r_index].to_owned();

            if r_index != 0 && file_space[r_index - 1] != current_symbol {
                move_forward(
                    r_index,
                    &mut file_space,
                    &current_symbol,
                    current_symbol_end,
                );
            }
        }
    }

    println!("Part 2: {}", calculate_checksum(&file_space));
}

fn move_forward(
    start_group: usize,
    file_space: &mut [String],
    current_symbol: &String,
    current_symbol_end: usize,
) {
    let mut free_len: usize = 0;

    'searchEmpty: for i in 0..start_group {
        if file_space[i] == "." {
            free_len += 1;

            if free_len > current_symbol_end - start_group {
                #[allow(clippy::needless_range_loop)]
                for j in start_group..=current_symbol_end {
                    file_space[j] = ".".to_owned();
                }

                #[allow(clippy::needless_range_loop)]
                for j in (i - free_len + 1)..=i {
                    file_space[j] = current_symbol.to_owned();
                }

                break 'searchEmpty;
            }
        } else {
            free_len = 0;
        }
    }
}

fn calculate_checksum(v: &[String]) -> usize {
    let mut checksum = 0;

    for (index, char) in v.iter().enumerate() {
        if char != "." {
            let num = str::parse::<usize>(char).unwrap();
            checksum += num * index;
        }
    }

    checksum
}
