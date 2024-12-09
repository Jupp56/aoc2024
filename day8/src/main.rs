use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (i, row) in chars.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '.' {
                match antennas.get_mut(c) {
                    Some(x) => x.push((i, j)),
                    None => {
                        antennas.insert(*c, vec![(i, j)]);
                    }
                }
            }
        }
    }

    part_1(&chars, &antennas);
    part_2(&chars, &antennas);
}

fn part_1(chars: &[Vec<char>], antennas: &HashMap<char, Vec<(usize, usize)>>) {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();


    for antenna_set in antennas {
        for i in 0..antenna_set.1.len() {
            for j in i + 1..antenna_set.1.len() {
                for row in 0..chars.len() {
                    for col in 0..chars[0].len() {
                        let a1 = antenna_set.1[i];
                        let a2 = antenna_set.1[j];

                        if (
                            row as isize + ((a1.0 as isize - row as isize) * 2),
                            col as isize + ((a1.1 as isize - col as isize) * 2),
                        ) == (a2.0 as isize, a2.1 as isize)
                            || (
                                row as isize + ((a2.0 as isize - row as isize) * 2),
                                col as isize + ((a2.1 as isize - col as isize) * 2),
                            ) == (a1.0 as isize, a1.1 as isize)
                        {
                            antinodes.insert((row, col));
                        }
                    }
                }
            }
        }
    }

    println!("Part1: {}", antinodes.len());
}

fn part_2(chars: &[Vec<char>], antennas: &HashMap<char, Vec<(usize, usize)>>) {
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    for antenna_set in antennas {
        for i in 0..antenna_set.1.len() {
            for j in i + 1..antenna_set.1.len() {
                let point_1 = antenna_set.1[i];
                let point_2 = antenna_set.1[j];
                let point_1 = (point_1.0 as isize, point_1.1 as isize);
                let point_2 = (point_2.0 as isize, point_2.1 as isize);

                let (distance_x, distance_y) = (point_1.0 - point_2.0, point_1.1 - point_2.1);

                let mut x = point_1.0;
                let mut y = point_1.1;

                while x >= 0 && x < chars.len() as isize && y >= 0 && y < chars[0].len() as isize {
                    antinodes.insert((x as usize, y as usize));
                    x += distance_x;
                    y += distance_y;
                }

                x = point_2.0;
                y = point_2.1;

                while x >= 0 && x < chars.len() as isize && y >= 0 && y < chars[0].len() as isize {
                    antinodes.insert((x as usize, y as usize));
                    x -= distance_x;
                    y -= distance_y;
                }
            }
        }
    }

    println!("Part2: {}", antinodes.len());
}
