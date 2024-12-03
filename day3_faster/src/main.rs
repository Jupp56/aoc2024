use std::{fs::File, io::Read, time::Instant};

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
    /// No dont encountered
    Start,
    /// Dont encountered before - only parse do at this point
    StartDont,
    /// "m"
    M,
    /// "mu"
    Mu,
    /// "mul"
    Mul,
    /// "mul("
    MulBracket,
    /// "mul(<number>", where number has up to three digits already
    FirstNumber,
    /// "mul(<number>,"
    Comma,
    /// "mul(<number>,<othernumber>", where othernumber has up to three digits already
    SecondNumber,
    /// "d"
    DFromStart,
    /// "do"
    DoFromStart,
    /// "do("
    DoBracketFromStart,
    /// encountered a "d," but from StartDont. We need to return to StartDont and only consider "do()" from here
    DFromStartDont,
    /// encountered a "do," but from StartDont. We need to return to StartDont and only consider "do()" from here
    DoFromStartDont,
    /// encountered a "do(" , but from StartDont.We need to return to StartDont.
    DoBracketFromStartDont,
    /// "don"
    Don,
    /// "don'"
    DonH,
    /// "don't"
    DonHt,
    /// "don't("
    DonHtBrackO,
}

fn part_2(input: &str) {
    let start = Instant::now();
    let mut state = State::Start;

    let mut first_number = 0;
    let mut second_number = 0;
    let mut num_digits = 0;

    let mut total = 0;

    // Iterate over the chars from the input
    for token in input.chars() {
        // If an expression we were in did not complete, we can try again from Start or StartDont via continue, as to not lose a token.
        // Example: take the expression "domul(2,3)". We do not match '(' or 'n' after the "do", because we find 'm'.
        // If it were not for this loop, we would stop processing the 'm' and continue with 'u' and we would loose the mul(2,3) expression.
        // There is a break statement at the end, so if we dont manually continue, we dont loop.
        loop {
            match state {
                State::Start => match token {
                    'm' => {
                        state = State::M;
                    }
                    'd' => {
                        state = State::DFromStart;
                    }
                    _ => {}
                },
                State::StartDont => {
                    if token == 'd' {
                        state = State::DFromStartDont;
                    }
                }
                State::M => {
                    if token == 'u' {
                        state = State::Mu;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::Mu => {
                    if token == 'l' {
                        state = State::Mul;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::Mul => {
                    if token == '(' {
                        state = State::MulBracket;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::MulBracket => {
                    if is_number(token) {
                        state = State::FirstNumber;
                        num_digits = 1;
                        first_number += token as u32 - 48
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::FirstNumber => {
                    if num_digits < 3 && is_number(token) {
                        num_digits += 1;
                        first_number *= 10;
                        first_number += token as u32 - 48
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
                    if is_number(token) {
                        state = State::SecondNumber;
                        num_digits = 1;
                        second_number += token as u32 - 48
                    } else {
                        first_number = 0;
                        num_digits = 0;
                        state = State::Start;
                        continue;
                    }
                }
                State::SecondNumber => {
                    if num_digits < 3 && is_number(token) {
                        num_digits += 1;
                        second_number *= 10;
                        second_number += token as u32 - 48
                    } else if token == ')' {
                        state = State::Start;
                        num_digits = 0;
                        total += first_number * second_number;
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
                State::DFromStart if token == 'o' => {
                    state = State::DoFromStart;
                }
                State::DFromStart => {
                    state = State::Start;
                    continue;
                }
                State::DoFromStart => match token {
                    'n' => {
                        state = State::Don;
                    }
                    '(' => {
                        state = State::DoBracketFromStart;
                    }
                    _ => {
                        state = State::Start;
                        continue;
                    }
                },
                State::DoBracketFromStart => {
                    if token == ')' {
                        state = State::Start;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
                State::DFromStartDont => {
                    if token == 'o' {
                        state = State::DoFromStartDont;
                    } else {
                        state = State::StartDont;
                        continue;
                    }
                }
                State::DoFromStartDont => match token {
                    '(' => {
                        state = State::DoBracketFromStartDont;
                    }
                    _ => {
                        state = State::StartDont;
                        continue;
                    }
                },
                State::DoBracketFromStartDont => {
                    if token == ')' {
                        state = State::Start;
                    } else {
                        state = State::StartDont;
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
                        state = State::StartDont;
                    } else {
                        state = State::Start;
                        continue;
                    }
                }
            }

            break;
        }
    }

    let end = Instant::now();
    println!("Part 2 took: {}", (end - start).as_micros());

    println!("Total part 2: {total}");
}

#[inline]
fn is_number(c: char) -> bool {
    c as u32 >= 48 && c as u32 <= 57
}
