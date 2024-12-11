use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut map: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        let mut v = Vec::new();
        for char in line.chars() {
            v.push(char as usize - 48);
        }
        map.push(v);
    }

    // starts
    let mut res_part_1 = 0;
    let mut res_part_2 = 0;

    let mut reachable = HashSet::new();

    for line in 0..map.len() {
        for col in 0..map[0].len() {
            if map[line][col] == 0 {
                search_part_1(&map, line, col, &mut reachable);
                res_part_1 += reachable.len();
                reachable.clear();

                res_part_2 += search_part_2(&map, line, col);
            }
        }
    }

    println!("Part 1: {res_part_1}, Part 2: {res_part_2}");
}

fn search_part_1(map: &[Vec<usize>], line: usize, col: usize, r: &mut HashSet<(usize, usize)>) {
    let current_val = map[line][col];

    if current_val == 9 {
        r.insert((line, col));
    }

    if line > 0 && map[line - 1][col] == current_val + 1 {
        search_part_1(map, line - 1, col, r);
    }

    if line < map.len() - 1 && map[line + 1][col] == current_val + 1 {
        search_part_1(map, line + 1, col, r);
    }

    if col > 0 && map[line][col - 1] == current_val + 1 {
        search_part_1(map, line, col - 1, r);
    }

    if col < map[0].len() - 1 && map[line][col + 1] == current_val + 1 {
        search_part_1(map, line, col + 1, r);
    }
}

fn search_part_2(map: &[Vec<usize>], line: usize, col: usize) -> usize {
    let current_val = map[line][col];

    if current_val == 9 {
        return 1;
    }

    let mut c = 0;

    if line > 0 && map[line - 1][col] == current_val + 1 {
        c += search_part_2(map, line - 1, col);
    }

    if line < map.len() - 1 && map[line + 1][col] == current_val + 1 {
        c += search_part_2(map, line + 1, col);
    }

    if col > 0 && map[line][col - 1] == current_val + 1 {
        c += search_part_2(map, line, col - 1);
    }

    if col < map[0].len() - 1 && map[line][col + 1] == current_val + 1 {
        c += search_part_2(map, line, col + 1);
    }

    c
}
