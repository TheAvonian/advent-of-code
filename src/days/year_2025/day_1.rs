#![allow(dead_code, unused)]

use itertools::Itertools;

pub fn run_day(input: String) {
    let rotations: Vec<i32> = input
        .lines()
        .map(|l| {
            let mut chars = l.chars();
            let char = chars.next().unwrap();
            let parsed = chars.join("").parse::<i32>().unwrap();
            if char.eq(&'R') {
                parsed
            } else {
                -parsed
            }
        })
        .collect();

    const MAX: i32 = 99i32;
    let end = rotations.iter().fold((0, 50), |(zeroes, rot), x| {
        let new_rot = (rot + x).rem_euclid(MAX + 1);
        if new_rot == 0 {
            return (zeroes + 1, new_rot);
        }

        (zeroes, new_rot)
    });
    let mut part1 = end.0;

    println!("Part 1: {}", part1);

    let end = rotations.iter().fold((0, 50), |(zeroes, rot), x| {
        let sub = (rot + x);
        let new_rot = sub.rem_euclid(MAX + 1);
        let mut zero_count = sub.div_euclid(MAX + 1).abs();
        if new_rot == 0 && zero_count == 0 && sub >= 0 {
            zero_count = 1;
        }
        if sub < 0 && rot == 0 {
            zero_count -= 1;
        }
        if sub < 0 && new_rot == 0 {
            zero_count += 1;
        }

        (zeroes + zero_count, new_rot)
    });

    let mut part2 = end.0;

    println!("Part 2: {}", part2);
}
