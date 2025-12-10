#![allow(dead_code, unused)]

use std::{
    collections::{BTreeSet, HashMap},
    ops::Rem,
};

use itertools::Itertools;
use split_every::SplitEveryImpl;

pub fn run_day(input: String) {
    let ranges = input
        .split(',')
        .map(|id| {
            let ids = id.split('-').collect_tuple::<(&str, &str)>().unwrap();
            (ids.0.parse::<i64>().unwrap(), ids.1.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();

    let end = ranges.iter().fold(0, |acc, (low, high)| {
        let mut count = 0;
        for i in (*low..(*high + 1)) {
            let total = i.to_string();
            let left = &total[..total.len() / 2];
            let right = &total[total.len() / 2..];
            if left.eq(right) {
                count += i;
            }
        }
        acc + count
    });
    let mut part1 = end;

    println!("Part 1: {}", part1);

    let mut all_babies: HashMap<i64, bool> = HashMap::new();

    let pattern = "".to_string();
    let end = ranges.iter().fold(0, |acc, (low, high)| {
        let mut count = 0;
        'outer: for i in (*low..(*high + 1)) {
            if let Some(v) = all_babies.get(&i) {
                if *v {
                    println!("FOUND DUPLICATE {}", i);
                    count += i;
                } else {
                    continue;
                }
            }
            let total = i.to_string();
            let divisions = total.len();
            'inner: for d in (2..=divisions) {
                if divisions.rem(d) != 0 {
                    continue;
                }
                let split_amount = divisions / d;

                let mut cursor = split_amount;
                let first = &total[0..cursor];
                while cursor + split_amount <= divisions {
                    let slice = &total[cursor..(cursor + split_amount)];
                    if !slice.eq(first) {
                        continue 'inner;
                    }

                    cursor += split_amount;
                }

                count += i;
                all_babies.insert(i, true);
                continue 'outer;
            }
            all_babies.insert(i, false);
        }
        acc + count
    });

    let mut part2 = end;

    println!("Part 2: {}", part2);
}
