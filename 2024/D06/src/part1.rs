use crate::common::{next_block, Dir};
use std::collections::HashSet;

pub fn solve(input: &str) {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let mut current_pos = grid
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            if let Some(x) = line.iter().position(|chr| *chr == '^') {
                Some((y, x))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, usize)>>()[0];

    let mut direction = Dir::UP;
    let mut marked = HashSet::new();

    loop {
        marked.insert(current_pos);

        if current_pos.0 == 0 || current_pos.1 == 0 {
            break;
        }

        let (next_y, next_x) = next_block(&current_pos, &direction);

        if next_y == grid.len() || next_x == grid[next_y].len() {
            break;
        }

        if grid[next_y][next_x] == '#' {
            direction = direction.next();
            continue;
        }

        current_pos = (next_y, next_x);
    }

    println!("distinct positions: {}", marked.len());
}
