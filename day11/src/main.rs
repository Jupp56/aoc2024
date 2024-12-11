use std::{collections::HashMap, fs::File, io::Read};

use num_bigint::BigUint;

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let stones: Vec<usize> = input.split(' ').map(|x| str::parse(x).unwrap()).collect();

    println!(
        "Part 1: {}, Part 2: {}",
        calc_stones(&stones, 25),
        calc_stones(&stones, 75)
    );
}

fn calc_stones(stones: &[usize], blinks: usize) -> BigUint {
    let mut cache: HashMap<(usize, usize), BigUint> = HashMap::new();
    let mut stones_to_do = Vec::new();

    for stone in stones {
        stones_to_do.push((*stone, blinks));
    }

    while let Some((stone, len_left)) = stones_to_do.pop() {
        if len_left == 0 {
            cache.insert((stone, 0), BigUint::from(1_usize));
            continue;
        }

        if cache.contains_key(&(stone, len_left)) {
            continue;
        }

        if stone == 0 {
            if let Some(r) = cache.get(&(1, len_left - 1)) {
                cache.insert((stone, len_left), r.clone());
            } else {
                stones_to_do.push((stone, len_left));
                stones_to_do.push((1, len_left - 1));
            }
        } else if (stone.ilog10() + 1) % 2 == 0 {
            let num_digits = (stone.ilog10() + 1) as usize;
            let stone_str: String = stone.to_string();

            let first_num: usize = str::parse(&stone_str[0..num_digits / 2]).unwrap();
            let second_num: usize = str::parse(&stone_str[num_digits / 2..]).unwrap();

            let cache_res_first = cache.get(&(first_num, len_left - 1));
            let cache_res_second = cache.get(&(second_num, len_left - 1));

            if let (Some(first), Some(second)) = (cache_res_first, cache_res_second) {
                cache.insert((stone, len_left), first + second);
            } else {
                stones_to_do.push((stone, len_left));

                if cache_res_first.is_none() {
                    stones_to_do.push((first_num, len_left - 1));
                }
                if cache_res_second.is_none() {
                    stones_to_do.push((second_num, len_left - 1));
                }
            }
        } else {
            let next_stone = stone * 2024;

            if let Some(r) = cache.get(&(next_stone, len_left - 1)) {
                cache.insert((stone, len_left), r.clone());
            } else {
                stones_to_do.push((stone, len_left));
                stones_to_do.push((next_stone, len_left - 1));
            }
        }
    }

    let mut total: BigUint = BigUint::from(0_usize);

    for stone in stones {
        total += cache.get(&(*stone, blinks)).unwrap();
    }

    total
}
