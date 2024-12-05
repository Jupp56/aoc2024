use std::{cmp::Ordering, fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input/input")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut s = input.split("\n\n");
    let rule_string = s.next().unwrap().to_owned();

    let update_string: String = s.next().unwrap().to_owned();

    let mut rules = Vec::new();

    for line in rule_string.lines() {
        let mut s = line.split('|');
        let a = str::parse::<u32>(s.next().unwrap()).unwrap();
        let b = str::parse::<u32>(s.next().unwrap()).unwrap();

        rules.push((a, b));
    }

    let mut updates: Vec<Vec<u32>> = Vec::new();

    for line in update_string.lines() {
        updates.push(
            line.split(',')
                .map(|x| str::parse::<u32>(x).unwrap())
                .collect(),
        );
    }

    part_1(&rules, &updates);
    part_2(&rules, updates);
}

fn part_1(rules: &[(u32, u32)], updates: &[Vec<u32>]) {
    let mut sum = 0;

    'outer: for update in updates {
        for rule in rules {
            if update_wrong(update, rule) {
                continue 'outer;
            }
        }
        sum += update[(update.len() - 1) / 2];
    }

    println!("sum: {sum}");
}
fn part_2(rules: &[(u32, u32)], updates: Vec<Vec<u32>>) {
    let mut sum = 0;

    for mut update in updates {
        let mut marked_wrong = false;
        for rule in rules {
            if update.contains(&rule.0)
                && update.contains(&rule.1)
                && update.iter().position(|x| x == &rule.0).unwrap()
                    > update.iter().position(|x| x == &rule.1).unwrap()
            {
                   update.sort_by(|a, b| {
                    if rules.iter().any(|(f, s)| f == a && s == b) {
                        Ordering::Greater
                    } else if rules.iter().any(|(f, s)| f == b && s == a) {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                });

                marked_wrong = true;
            }
        }
        if marked_wrong {
            sum += update[(update.len() - 1) / 2];
        }
    }

    println!("sum: {sum}");
}

fn update_wrong(update: &[u32], rule: &(u32, u32)) -> bool {
    update.contains(&rule.0)
        && update.contains(&rule.1)
        && update.iter().position(|x| x == &rule.0).unwrap()
            > update.iter().position(|x| x == &rule.1).unwrap()
}
