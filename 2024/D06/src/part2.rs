use crate::common::{next_block, Dir};
use itertools::{Itertools, ZipLongest};
use std::{collections::HashMap, ops::Range};

fn get_range(
    pos: &(usize, usize),
    direction: &Dir,
    grid: &Vec<Vec<char>>,
) -> ZipLongest<Range<usize>, Range<usize>> {
    let (y, x) = pos;

    match direction {
        Dir::UP => (*y..*y).zip_longest(*x + 1..grid[*y].len()),
        Dir::RIGHT => (*y + 1..grid.len()).zip_longest(*x..*x),
        Dir::DOWN => (*y..*y).zip_longest(grid[*y].len()..*x - 1),
        Dir::LEFT => (grid.len()..*y - 1).zip_longest(*x..*x),
    }
}

fn check_obstacle(
    pos: &(usize, usize),
    dir: &Dir,
    obstacles: &HashMap<(usize, usize), Dir>,
    grid: &Vec<Vec<char>>,
) -> bool {
    let range = get_range(pos, dir, grid);

    println!("range is {:?}", range.clone().collect());

    for pos2 in range {
        println!("pos2 is {pos2:?}");
        if let Some(dir2) = obstacles.get(&pos2) {
            if *dir2 != dir.next() {
                continue;
            }

            let range2 = get_range(&pos2, &dir2, grid);

            for pos3 in range2 {
                if let Some(dir3) = obstacles.get(&pos3) {
                    if *dir3 != dir2.next() {
                        continue;
                    }

                    return true;
                }
            }
        }
    }
    false
}

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
    let mut obstacles = HashMap::new();

    // get obstacles
    loop {
        if current_pos.0 == 0 || current_pos.1 == 0 {
            break;
        }

        let (next_y, next_x) = next_block(&current_pos, &direction);

        if next_y == grid.len() || next_x == grid[next_y].len() {
            break;
        }

        if grid[next_y][next_x] == '#' {
            obstacles.insert((current_pos.0, current_pos.1), direction);
            direction = direction.next();
            continue;
        }

        current_pos = (next_y, next_x);
    }

    println!("ass: {obstacles:?}");

    let mut total = 0;

    for (pos, dir) in obstacles.iter() {
        if check_obstacle(pos, dir, &obstacles, &grid) {
            println!("true pos is {pos:?}");

            total += 1;
            continue;
        }
    }

    println!("positions are {total}");
}
