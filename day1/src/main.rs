use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input").unwrap().read_to_string(&mut input).unwrap();

    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in input.lines() {
        let mut s = line.split("   ");
        list1.push(str::parse(s.next().unwrap()).unwrap());
        list2.push(str::parse(s.next().unwrap()).unwrap());
    }

    part1(list1.clone(), list2.clone());
    part2(&list1, &list2);
}

fn part1(mut list1: Vec<u32>, mut list2: Vec<u32>) {
    list1.sort();
    list2.sort();

    let mut distance_total = 0;

    for (first, second) in list1.iter().zip(list2) {
        distance_total += first.abs_diff(second);
    }

    println!("Part 1: {distance_total}");
}

fn part2(list1: &Vec<u32>, list2: &Vec<u32>) {
    let mut total = 0;

    for elem in list1 {
        let mut counter = 0;
        for l2 in list2 {
            if l2 == elem {
                counter += 1;
            }
        } 

        total += elem * counter;
    }

    println!("Part 2: {total}");
}
