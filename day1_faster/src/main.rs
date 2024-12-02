use std::{fs::File, io::Read, time::Instant};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut list1: Vec<usize> = Vec::new();
    let mut list2: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut s = line.split("   ");
        list1.push(str::parse(s.next().unwrap()).unwrap());
        list2.push(str::parse(s.next().unwrap()).unwrap());
    }

    part1(list1.clone(), list2.clone());

    let start = Instant::now();
    let total = part2(&list1, &list2);
    let end = Instant::now();
    println!("Part 2: {total}");

    println!("Part 2 took {} microseconds", (end - start).as_micros());
}

fn part1(mut list1: Vec<usize>, mut list2: Vec<usize>) {
    list1.sort();
    list2.sort();

    let mut distance_total = 0;

    for (first, second) in list1.iter().zip(list2) {
        distance_total += first.abs_diff(second);
    }

    println!("Part 1: {distance_total}");
}

fn part2(list1: &[usize], list2: &[usize]) -> usize {
    let mut arr: [usize; 99999] = [0; 99999];

    let mut total = 0;

    for elem in list2 {
        arr[*elem] += 1;
    }

    for elem in list1 {
        total += elem * arr[*elem];
    }

    total
}