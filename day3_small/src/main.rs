use std::{fs::File, io::Read};

use regex::Regex;

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let re = Regex::new(r"(?m)mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();

    part_1(&input, &re);
    part_2(&input, &re);
}

fn part_1(input: &str, re: &Regex) {
    let total = parse_muls(re, input);

    println!("Total: {total}");
}

fn parse_muls(re: &Regex, input: &str) -> u32 {
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
    total
}

fn part_2(input: &str, re: &Regex) {
    let mut first_section = true;
    let mut total = 0;
    for dont_section in input.split("don't()") {
        if first_section {
            first_section = false;
            total += parse_muls(re, dont_section);
            continue;
        }

        let mut first_do_section = true;
        for do_section in dont_section.split("do()") {
            if first_do_section {
                first_do_section = false;
                continue;
            }

            total += parse_muls(re, do_section);
        }
    }

    println!("Total part 2: {total}");
}
