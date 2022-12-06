#![allow(dead_code, unused)]

use itertools::Itertools;

pub fn run_day(input: String) {
    let input = input.chars().map_into::<String>().collect_vec();

    let part1 = input
        .iter()
        .tuple_windows()
        .position(|(a, b, c, d)| vec![a, b, c, d].iter().all_unique())
        .unwrap()
        + 4;

    println!("Part 1: {}", part1);

    let part2 = (13..input.len())
        .position(|i| input.iter().skip(i - 13).take(14).all_unique())
        .unwrap()
        + 14;

    println!("Part 2: {}", part2);
}
