#![allow(dead_code, unused)]

use std::ops::Mul;

use itertools::Itertools;

pub fn run_day(input: String) {
    let mut numbers: Vec<Vec<i64>> = input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter(|x| !x.is_empty())
                .map(|x| {
                    if let Ok(v) = x.parse::<i64>() {
                        v
                    } else {
                        match x.chars().nth(0) {
                            Some('+') => 0,
                            Some('*') => 1,
                            _ => panic!("AH"),
                        }
                    }
                })
                .collect::<Vec<i64>>()
        })
        .collect();
    let additions = &numbers[numbers.len() - 1];
    let numbers = &numbers[..numbers.len() - 1];
    let mut part1 = 0;

    for i in 0..numbers[0].len() {
        let mut tmp = numbers[0][i];
        for j in 1..numbers.len() {
            if additions[i] == 0 {
                tmp += numbers[j][i];
            } else {
                tmp *= numbers[j][i];
            }
        }
        part1 += tmp;
    }

    println!("Part 1: {}", part1);
    let mut part2 = 0;

    let diffs = input
        .lines()
        .rev()
        .next()
        .expect("ONE BOY")
        .chars()
        .fold(vec![], |mut acc, c| {
            if !c.is_whitespace() {
                acc.push((if c.eq(&'+') { 0 } else { 1 }, 1usize));
                acc
            } else {
                let len = acc.len() - 1;
                acc[len].1 += 1;
                acc
            }
        });
    let lines = input
        .lines()
        .take(input.lines().count())
        .map(|x| x.chars().collect_vec())
        .collect_vec();

    let mut numbers = vec![];
    let mut total_i = 0;
    for diff in diffs {
        let mut i = diff.1;
        numbers.push(diff.0);
        let last = numbers.len() - 1;
        while i > 0 {
            i -= 1;

            let mut number = 0;
            for n in 0..lines.len() {
                let c = lines[n][i + total_i];
                if let Some(v) = c.to_digit(10) {
                    number += v as u64 * (10u64.pow(lines.len() as u32 - 1 - n as u32))
                } else {
                    number /= 10;
                }
            }

            if number == 0 {
                continue;
            }

            if diff.0 == 0 {
                numbers[last] += number;
            } else {
                numbers[last] *= number;
            }
        }
        total_i += diff.1;
    }

    part2 = numbers.iter().sum();

    println!("Part 2: {}", part2);
}
