use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn part1(input: String) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        list1.push(parts[0]);
        list2.push(parts[1]);
    }

    list1.sort();
    list2.sort();

    let sum: i32 = list1
        .into_iter()
        .zip(list2)
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();

    println!("Total distance is: {sum}");
}

fn part2(input: String) {
    let mut dict1: HashMap<i32, i32> = HashMap::new();
    let mut dict2: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        let (num1, num2) = (parts[0], parts[1]);

        match dict1.get(&num1) {
            Some(count) => dict1.insert(num1, count + 1),
            None => dict1.insert(num1, 1),
        };
        match dict2.get(&num2) {
            Some(count) => dict2.insert(num2, count + 1),
            None => dict2.insert(num2, 1),
        };
    }

    let similarity: i32 = dict1
        .into_iter()
        .map(|(k, v)| k * v * dict2.get(&k).unwrap_or(&0))
        .sum();

    println!("Similarity is: {similarity}");
}

fn main() {
    let input1 = fs::read_to_string("inputs/01/1.txt").unwrap();
    let input2 = fs::read_to_string("inputs/01/2.txt").unwrap();

    let now = Instant::now();
    part1(input1);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");

    let now = Instant::now();
    part2(input2);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");
}
