use std::{fs, time::Instant};

mod common;
mod part1;
mod part2;

fn main() {
    let input = fs::read_to_string("inputs/06.txt").unwrap();

    let now = Instant::now();
    crate::part1::solve(&input);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");

    // let now = Instant::now();
    // part2(&input);
    // let elapsed = now.elapsed();
    // println!("part2 took: {elapsed:.2?}");
}
