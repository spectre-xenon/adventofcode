use std::{char, fs, time::Instant};

fn try_parse_num(input: &Vec<char>, pointer: &mut usize, first_num: bool) -> Option<u32> {
    let mut num_temp = String::new();
    let current_pointer = *pointer;
    let break_chr = if first_num { ',' } else { ')' };

    loop {
        if input[*pointer] == break_chr {
            break;
        }

        if *pointer - current_pointer > 2 || input[*pointer] == ' ' {
            return None;
        }

        num_temp.push(input[*pointer]);

        *pointer += 1;
    }

    match num_temp.parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
}

fn try_parse(input: &Vec<char>, pointer: &mut usize) -> Option<(u32, u32)> {
    let mut instruction = String::new();
    let mul = vec!['m', 'u', 'l'];

    for i in 0..3 {
        let chr = input[*pointer];

        if chr != mul[i] {
            return None;
        }

        instruction.push(input[*pointer]);
        *pointer += 1;
    }

    if input[*pointer] != '(' {
        return None;
    }

    *pointer += 1;

    let num1 = try_parse_num(input, pointer, true)?;
    *pointer += 1;
    let num2 = try_parse_num(input, pointer, false)?;

    if input[*pointer] != ')' {
        return None;
    }

    Some((num1, num2))
}

fn part1(input: &str) {
    let input: Vec<char> = input.chars().collect();
    let mut mul_vector: Vec<(u32, u32)> = Vec::new();
    let mut pointer = 0;

    loop {
        if pointer == input.len() {
            break;
        }

        if input[pointer] == 'm' {
            if let Some(num_tuple) = try_parse(&input, &mut pointer) {
                mul_vector.push(num_tuple);
            }
        }

        pointer += 1;
    }

    let sum: u32 = mul_vector.into_iter().map(|(n1, n2)| n1 * n2).sum();

    println!("sum of mul instructions is: {sum}");
}

fn try_parse_conditional(input: &Vec<char>, pointer: &mut usize) -> Option<bool> {
    let mut instruction = String::new();
    let mut is_do = false;
    let dont = vec!['d', 'o', 'n', '\'', 't'];

    for i in 0..5 {
        let chr = input[*pointer];

        if i == 2 && chr == '(' {
            is_do = true;
            break;
        }

        if chr != dont[i] {
            return None;
        }

        instruction.push(input[*pointer]);
        *pointer += 1;
    }

    if is_do && input[*pointer + 1] == ')' {
        return Some(true);
    }

    if input[*pointer] == '(' && input[*pointer + 1] == ')' {
        return Some(false);
    }

    None
}

fn part2(input: &str) {
    let input: Vec<char> = input.chars().collect();
    let mut mul_vector: Vec<(u32, u32)> = Vec::new();
    let mut pointer = 0;

    let mut enabled = true;

    loop {
        if pointer == input.len() {
            break;
        }

        if input[pointer] == 'd' {
            if let Some(condition) = try_parse_conditional(&input, &mut pointer) {
                enabled = condition;
            };
        }

        if input[pointer] == 'm' && enabled {
            if let Some(num_tuple) = try_parse(&input, &mut pointer) {
                mul_vector.push(num_tuple);
            }
        }

        pointer += 1;
    }

    let sum: u32 = mul_vector.into_iter().map(|(n1, n2)| n1 * n2).sum();

    println!("sum of mul instructions is: {sum}");
}

fn main() {
    let input = fs::read_to_string("inputs/03.txt").unwrap();

    let now = Instant::now();
    part1(&input);
    let elapsed = now.elapsed();
    println!("part1 took: {elapsed:.2?}");

    let now = Instant::now();
    part2(&input);
    let elapsed = now.elapsed();
    println!("part2 took: {elapsed:.2?}");
}
