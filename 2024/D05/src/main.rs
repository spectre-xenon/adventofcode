use std::{collections::HashMap, fs, ops::Index, time::Instant};

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, Vec<Vec<&str>>) {
    let splitted: Vec<&str> = input.split("\n\n").collect();
    let splitted_rules: Vec<(&str, &str)> = splitted[0]
        .lines()
        .map(|l| {
            let pair: Vec<&str> = l.split("|").collect();
            (pair[0], pair[1])
        })
        .collect();

    let updates: Vec<Vec<&str>> = splitted[1]
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .collect();
    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();

    for (k, v) in splitted_rules.into_iter() {
        match rules.get_mut(k) {
            Some(value) => {
                value.push(v);
            }
            None => {
                rules.insert(k, vec![v]);
            }
        };
    }

    (rules, updates)
}

fn part1(input: &str) {
    let (rules, updates) = parse_input(input);

    let mut sum = 0;
    for line in updates.iter() {
        let mut correct_flag = true;
        let mut previous = Vec::new();

        for num in line.iter() {
            if !correct_flag {
                break;
            }

            match rules.get(num) {
                Some(values) => {
                    for value in values.iter() {
                        if previous.contains(value) {
                            correct_flag = false;
                            break;
                        }
                    }
                    previous.push(num);
                }

                None => {
                    previous.push(num);
                }
            }
        }

        if correct_flag {
            sum += line[line.len() / 2].parse::<u32>().unwrap();
        }
    }

    println!("sum is: {sum}");
}

fn part2(input: &str) {
    let (rules, mut updates) = parse_input(input);

    let mut sum = 0;
    for line in updates.iter_mut() {
        let mut add_incorrect = false;
        let mut swap_flag = false;
        let mut incorrect_index: usize = 0;
        let mut previous = Vec::new();
        let mut pointer = 0;

        loop {
            if pointer == line.len() {
                break;
            }

            let num = line[pointer];

            match rules.get(num) {
                Some(values) => {
                    for value in values.iter() {
                        if previous.contains(value) {
                            add_incorrect = true;
                            swap_flag = true;
                            incorrect_index = previous.iter().position(|n| n == value).unwrap();
                            ()
                        }
                    }
                    previous.push(num);
                }

                None => {
                    previous.push(num);
                }
            }

            if swap_flag {
                line.swap(pointer, incorrect_index);
                previous.clear();
                swap_flag = false;
                pointer = 0;
                continue;
            }
            pointer += 1;
        }

        if add_incorrect {
            sum += line[line.len() / 2].parse::<u32>().unwrap();
        }
    }

    println!("sum is: {sum}");
}

fn main() {
    let input = fs::read_to_string("inputs/05.txt").unwrap();

    let now = Instant::now();
    part1(&input);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");

    let now = Instant::now();
    part2(&input);
    let elapsed = now.elapsed();
    println!("part2 took: {elapsed:.2?}");
}
