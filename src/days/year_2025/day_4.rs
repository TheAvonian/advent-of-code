#![allow(dead_code, unused)]

use std::collections::{BTreeMap, BTreeSet};

pub fn run_day(input: String) {
    let mut part1 = 0;

    let map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| {
                if c.eq(&'@') {
                    return Some((i as i64, j as i64));
                }
                None
            })
        })
        .collect::<BTreeSet<(i64, i64)>>();

    part1 = map.iter().fold(0, |acc, (x, y)| {
        let mut papers = 0;
        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if i == *x && j == *y {
                    continue;
                }
                if map.contains(&(i, j)) {
                    papers += 1;
                    if papers >= 4 {
                        return acc;
                    }
                }
            }
        }

        acc + 1
    });

    println!("Part 1: {}", part1);
    let mut part2 = 0;

    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, c)| {
                if c.eq(&'@') {
                    return Some(((i as i64, j as i64), 0));
                }
                None
            })
        })
        .collect::<BTreeMap<(i64, i64), u8>>();
    let mut keys = map
        .iter()
        .map(|(a, b)| *a)
        .collect::<BTreeSet<(i64, i64)>>();

    for ((x, y), papers) in map.iter_mut() {
        for i in (x - 1)..=(x + 1) {
            for j in (y - 1)..=(y + 1) {
                if i == *x && j == *y {
                    continue;
                }
                if keys.contains(&(i, j)) {
                    *papers += 1;
                }
            }
        }
    }
    let max = (
        map.iter().max_by_key(|x| x.0 .0).expect("MAX").0 .0,
        map.iter().max_by_key(|x| x.0 .1).expect("MAX").0 .1,
    );

    let mut total = 0;
    for (x, y) in keys.iter() {
        let Some(papers) = map.get(&(*x, *y)) else {
            continue;
        };
        if *papers < 4 {
            total = remove_traverse(&mut map, (*x, *y), total);
        }
    }
    part2 = total;

    // for i in 0..max.0 {
    //     for j in 0..max.1 {
    //         if let Some(p) = map.get(&(i, j)) {
    //             print!("{}", p);
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     print!("\n");
    // }

    println!("Part 2: {}", part2);
}

fn remove_traverse(map: &mut BTreeMap<(i64, i64), u8>, (x, y): (i64, i64), mut acc: i64) -> i64 {
    let Some(val) = map.get_mut(&(x, y)) else {
        return acc;
    };

    if *val > 4 {
        *val -= 1;
        return acc;
    }
    map.remove(&(x, y));

    acc += 1;
    for i in (x - 1)..=(x + 1) {
        for j in (y - 1)..=(y + 1) {
            if i == x && j == y {
                continue;
            }
            if map.contains_key(&(i, j)) {
                acc = remove_traverse(map, (i, j), acc);
            }
        }
    }
    acc
}
