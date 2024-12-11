use std::{fs::File, io::Read, time::Instant};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Entry {
    Num(usize),
    Empty,
}

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut harddrive = Vec::with_capacity(input.len());

    let input: Vec<usize> = input.chars().map(|c| c as usize - 48).collect();

    for (index, number) in input.iter().enumerate() {
        if index % 2 == 0 {
            for _ in 0..*number {
                harddrive.push(Entry::Num(index / 2));
            }
        } else {
            for _ in 0..*number {
                harddrive.push(Entry::Empty);
            }
        }
    }

    let clone = harddrive.clone();

    part_1(clone);
    part_2(harddrive);

}

fn part_1(mut harddrive: Vec<Entry>) {
    let mut first_free = 0;
    for i in 0..harddrive.len() {
        let r_index = harddrive.len() - 1 - i;

        if let Entry::Num(_) = harddrive[r_index] {
            for j in first_free..r_index {
                if harddrive[j] == Entry::Empty {
                    harddrive[j] = harddrive[r_index];
                    if r_index != j {
                        harddrive[r_index] = Entry::Empty;
                    }
                    first_free = j;
                    break;
                }
            }
        }
    }

    println!("Part 1: {}", calculate_checksum(&harddrive));
}

fn part_2(mut harddrive: Vec<Entry>) {
    let mut current_symbol_end = harddrive.len() - 1;
    let mut current_symbol = 0;

    let mut free_by_length: [usize; 10] = [0; 10];

    for r_index in (0..harddrive.len()).rev() {
        if let Entry::Num(number) = harddrive[r_index] {
            if number != current_symbol {
                current_symbol_end = r_index;
            }

            current_symbol = number;

            if r_index != 0 {
                if let Entry::Num(n) = harddrive[r_index - 1] {
                    if n != current_symbol {
                        move_to_free_slot(
                            r_index,
                            &mut harddrive,
                            current_symbol,
                            current_symbol_end,
                            &mut free_by_length,
                        );
                        current_symbol = 0;
                    }
                } else {
                    move_to_free_slot(
                        r_index,
                        &mut harddrive,
                        current_symbol,
                        current_symbol_end,
                        &mut free_by_length,
                    );
                    current_symbol = 0;
                }
            }
        }
    }

    println!("Part 2: {}", calculate_checksum(&harddrive));
}

fn move_to_free_slot(
    start_of_group: usize,
    harddrive: &mut [Entry],
    current_symbol: usize,
    current_symbol_end: usize,
    free_by_length: &mut [usize; 10],
) {
    let mut free_len: usize = 0;

    let necessary_len = current_symbol_end - start_of_group + 1;

    let search_start_index = free_by_length[necessary_len]; //*free_by_length[0..=necessary_len].iter().max().unwrap();

    for i in search_start_index..start_of_group {
        if harddrive[i] == Entry::Empty {
            free_len += 1;

            if free_len >= necessary_len {
                #[allow(clippy::needless_range_loop)]
                for j in start_of_group..=current_symbol_end {
                    harddrive[j] = Entry::Empty;
                }

                #[allow(clippy::needless_range_loop)]
                for j in (i - free_len + 1)..=i {
                    harddrive[j] = Entry::Num(current_symbol);
                }

                for j in necessary_len..=9 {
                    if free_by_length[j] < i {
                        free_by_length[j] = i;
                    }
                }

                break;
            }
        } else {
            free_len = 0;
        }
    }
}

fn calculate_checksum(v: &[Entry]) -> usize {
    let mut checksum = 0;

    for (index, entry) in v.iter().enumerate() {
        if let Entry::Num(number) = entry {
            checksum += number * index;
        }
    }

    checksum
}
