#![allow(dead_code, unused)]

use itertools::Itertools;

pub fn run_day(input: String) {
    const SIZE_1: usize = 2;

    let mut part1 = input
        .lines()
        .fold(0, |acc, s| find_battery(s, SIZE_1) + acc);

    println!("Part 1: {}", part1);

    const SIZE_2: usize = 12;

    let mut part2 = input
        .lines()
        .fold(0, |acc, s| find_battery(s, SIZE_2) + acc);

    println!("Part 2: {}", part2);
}

fn find_battery(string: &str, max_size: usize) -> u64 {
    let numbers: Vec<u64> = string
        .chars()
        .map(|x| x.to_digit(10).expect("BAD PARSE") as u64)
        .collect();

    let mut biggest: Vec<u64> = vec![0; max_size];
    for (i, x) in (0..).zip(numbers.iter()) {
        for s in ((max_size as i32 - numbers.len() as i32 + i).max(0) as usize)..max_size {
            if x > &biggest[s] {
                biggest[s] = *x;
                biggest[s + 1..].fill(0);
                break;
            }
        }
    }

    biggest
        .iter()
        .map(u64::to_string)
        .join("")
        .parse::<u64>()
        .expect("BAD FORMAT PARSE")
}
