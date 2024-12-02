use std::{fs::File, io::Read};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Undecided,
}

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    part_1(&input);
    part_2(&input);
}

fn part_1(input: &str) {
    let mut safe_lines = 0;

    for line in input.lines() {
        let numbers: Vec<&str> = line.split(" ").collect();
        let numbers: Vec<u8> = numbers.iter().map(|n| str::parse(n).unwrap()).collect();

        if check_numbers(&numbers) {
            safe_lines += 1;
        }
    }

    println!("Safe lines part 1: {safe_lines}");
}

fn part_2(input: &str) {
    let mut safe_lines = 0;

    for line in input.lines() {
        let numbers: Vec<&str> = line.split(" ").collect();
        let numbers: Vec<u8> = numbers.iter().map(|n| str::parse(n).unwrap()).collect();

        if check_numbers(&numbers) {
            safe_lines += 1;
        } else {
            for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);
                if check_numbers(&numbers) {
                    safe_lines += 1;
                    break;
                }
            }
        }
    }

    println!("Safe lines part 2: {safe_lines}");
}

fn check_numbers(numbers: &[u8]) -> bool {
    let mut direction = Direction::Undecided;
    let mut last_number = 0;

    for number in numbers {
        if last_number == 0 {
            last_number = *number;
            continue;
        }

        if direction == Direction::Undecided {
            if *number > last_number {
                direction = Direction::Up;
            } else {
                direction = Direction::Down;
            }
        }

        if last_number.abs_diff(*number) > 3 {
            return false;
        }

        if direction == Direction::Up && *number < last_number
            || direction == Direction::Down && *number > last_number
            || *number == last_number
        {
            return false;
        }

        last_number = *number;
    }

    true
}
