use std::{fs::File, io::Read};

use regex::Regex;

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
    let re = Regex::new(r"(?m)mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();

    let mut results = Vec::new();

    for (_, [first, second]) in re.captures_iter(input).map(|x| x.extract()) {
        results.push((
            str::parse::<u32>(first).unwrap(),
            str::parse::<u32>(second).unwrap(),
        ));
    }

    let mut total = 0;

    for result in results {
        total += result.0 * result.1;
    }

    println!("Total: {total}");
}

enum State {
    Start,
    M,
    U,
    L,
    MulBrackO,
    FirstNumber,
    Comma,
    SecondNumber,
    D,
    Do,
    DoBrackO,
    Don,
    DonH,
    DonHt,
    DonHtBrackO,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

fn part_2(input: &str) {
    let instructions = parse_instructions(input);

    let mut total = 0;
    let mut muls_active = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if muls_active {
                    total += a * b;
                }
            }
            Instruction::Do => {
                muls_active = true;
            }
            Instruction::Dont => {
                muls_active = false;
            }
        }
    }

    println!("Total part 2: {total}");
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    let mut state = State::Start;

    let mut first_number = 0;
    let mut second_number = 0;
    let mut num_digits = 0;

    for token in input.chars() {
        loop {
            match state {
                State::Start => match token {
                    'm' => {
                        state = State::M;
                    }
                    'd' => {
                        state = State::D;
                    }
                    _ => {}
                },
                State::M => {
                    if token == 'u' {
                        state = State::U;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::U => {
                    if token == 'l' {
                        state = State::L;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::L => {
                    if token == '(' {
                        state = State::MulBrackO;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::MulBrackO => {
                    if NUMBERS.contains(&token) {
                        state = State::FirstNumber;
                        num_digits = 1;
                        first_number += str::parse::<u32>(&token.to_string()).unwrap();
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::FirstNumber => {
                    if num_digits < 3 && NUMBERS.contains(&token) {
                        num_digits += 1;
                        first_number *= 10;
                        first_number += str::parse::<u32>(&token.to_string()).unwrap();
                    } else if token == ',' {
                        state = State::Comma;
                        num_digits = 0;
                    } else {
                        num_digits = 0;
                        first_number = 0;
                        state = State::Start;
                        continue;
                    }
                }
                State::Comma => {
                    if NUMBERS.contains(&token) {
                        state = State::SecondNumber;
                        num_digits = 1;
                        second_number += str::parse::<u32>(&token.to_string()).unwrap();
                    } else {
                        first_number = 0;
                        num_digits = 0;
                        state = State::Start;
                        continue;
                    }
                }
                State::SecondNumber => {
                    if num_digits < 3 && NUMBERS.contains(&token) {
                        num_digits += 1;
                        second_number *= 10;
                        second_number += str::parse::<u32>(&token.to_string()).unwrap();
                    } else if token == ')' {
                        state = State::Start;
                        num_digits = 0;
                        instructions.push(Instruction::Mul(first_number, second_number));
                        first_number = 0;
                        second_number = 0;
                    } else {
                        num_digits = 0;
                        first_number = 0;
                        second_number = 0;
                        state = State::Start;
                        continue;
                    }
                }
                State::D => {
                    if token == 'o' {
                        state = State::Do;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::Do => match token {
                    'n' => {
                        state = State::Don;
                    }
                    '(' => {
                        state = State::DoBrackO;
                    }
                    _ => {
                        state = State::Start;
                        continue;
                    }
                },
                State::DoBrackO => {
                    if token == ')' {
                        state = State::Start;
                        instructions.push(Instruction::Do);
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::Don => {
                    if token == '\'' {
                        state = State::DonH;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::DonH => {
                    if token == 't' {
                        state = State::DonHt;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::DonHt => {
                    if token == '(' {
                        state = State::DonHtBrackO;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::DonHtBrackO => {
                    if token == ')' {
                        instructions.push(Instruction::Dont);
                        state = State::Start;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
            }

            break;
        }
    }
    instructions
}
