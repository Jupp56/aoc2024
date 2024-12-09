use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let measurements: Vec<(usize, Vec<usize>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");

            let result = str::parse::<usize>(split.next().unwrap()).unwrap();

            let operands = split
                .next()
                .unwrap()
                .split(' ')
                .map(|num| str::parse::<usize>(num).unwrap())
                .collect();

            (result, operands)
        })
        .collect();

    part_1(&measurements);
    part_2(&measurements);
}

fn part_1(measurements: &[(usize, Vec<usize>)]) {
    let mut sum = 0;

    for (result, measurements) in measurements {
        if parse_result(*result, measurements[0], &measurements[1..], false) {
            sum += result
        }
    }

    println!("Part 1: {sum}");
}

fn part_2(measurements: &[(usize, Vec<usize>)]) {
    let mut sum = 0;

    for (result, measurements) in measurements {
        if parse_result(*result, measurements[0], &measurements[1..], true) {
            sum += result
        }
    }

    println!("Part 2: {sum}");
}

fn parse_result(expected: usize, current: usize, v: &[usize], concat: bool) -> bool {
    if v.is_empty() {
        return expected == current;
    }

    if concat {
        let places = (v[0]).ilog10() + 1;

        let new_cur = current * 10_usize.pow(places);

        if parse_result(expected, new_cur + v[0], &v[1..], concat) {
            return true;
        }
    }
    if parse_result(expected, current * v[0], &v[1..], concat) {
        return true;
    }

    parse_result(expected, current + v[0], &v[1..], concat)
}
