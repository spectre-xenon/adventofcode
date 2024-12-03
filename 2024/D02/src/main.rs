use std::{fs, time::Instant};

fn is_safe(reports: &Vec<i32>) -> bool {
    let mut is_increasing = false;

    for i in 0..reports.len() - 1 {
        let diff = reports[i] - reports[i + 1];

        if i == 0 && diff < 0 {
            is_increasing = true;
        }

        if diff == 0 {
            return false;
        }
        if is_increasing && (diff < -3 || diff > -1) {
            return false;
        }
        if !is_increasing && (diff > 3 || diff < 1) {
            return false;
        }
    }

    true
}

fn part1(input: &str) {
    let mut safe = 0;

    for line in input.lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|r| r.parse().unwrap())
            .collect();

        if is_safe(&reports) {
            safe += 1
        }
    }

    println!("Safe reports number is: {safe}");
}

fn part2(input: &str) {
    let mut safe = 0;

    for line in input.lines() {
        let reports: Vec<i32> = line
            .split_whitespace()
            .map(|r| r.parse().unwrap())
            .collect();

        if is_safe(&reports) {
            safe += 1;
            continue;
        }

        for i in 0..reports.len() {
            let mut cloned = reports.clone();
            cloned.remove(i);

            if is_safe(&cloned) {
                safe += 1;
                break;
            }
        }
    }

    println!("Safe reports number is: {safe}");
}

fn main() {
    let input = fs::read_to_string("inputs/02.txt").unwrap();

    let now = Instant::now();
    part1(&input);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");

    let now = Instant::now();
    part2(&input);
    let elapsed = now.elapsed();
    println!("part2 took: {elapsed:.2?}");
}
