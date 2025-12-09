#![allow(dead_code, unused)]

use itertools::Itertools;

pub fn run_day(input: String) {
    let ranges: Vec<std::ops::Range<i128>> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (left, right) = x.split_once('-').expect("INVALID RANGE");
            // println!("RANGE {} AND {}", left, right);
            (left.parse::<i128>().expect("BAD PARSE")
                ..right.parse::<i128>().expect("BAD PARSE") + 1)
        })
        .collect();

    let checks = input
        .lines()
        .skip_while(|x| !x.is_empty())
        .skip(1)
        .map(|x| {
            // println!("CHECK {}", x);
            x.parse::<i128>().expect("SECOND BAS PARse")
        })
        .collect::<Vec<i128>>();

    let mut valids = 0;
    for i in &checks {
        // println!("CHECKING {}", i);
        for range in &ranges {
            if range.contains(i) {
                valids += 1;
                break;
            }
        }
    }
    let mut part1 = valids;

    println!("Part 1: {}", part1);

    let ranges: Vec<(i128, i128)> = input
        .lines()
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let (left, right) = x.split_once('-').expect("INVALID RANGE");
            (
                left.parse::<i128>().expect("BAD PARSE"),
                right.parse::<i128>().expect("BAD PARSE"),
            )
        })
        .collect();
    let mut part2 = 0;

    let mut good_range: Vec<(i128, i128)> = vec![];

    'add: for adding_range in &ranges {
        let mut ri = good_range.len() as i64 - 1;
        let mut expanding_range: Option<usize> = None;
        while ri >= 0 {
            let r = ri as usize;
            let active_range: (i128, i128) = good_range[r];

            // we are actively expanding
            if let Some(expanded_range) = expanding_range {
                let erange = &mut good_range[expanded_range];
                // if min of erange is bigger than max of current range then we're good
                if erange.0 > active_range.1 {
                    continue 'add;
                }

                // if min is within this active range then  set min of erange
                if erange.0 >= active_range.0 {
                    erange.0 = active_range.0;
                }

                // range is within or expanded into so delete
                good_range.remove(r);
                expanding_range = Some(expanded_range - 1);

                if r <= 0 {
                    continue 'add;
                }
            } else {
                if adding_range.1 >= active_range.1 {
                    // maximum of range bigger than current known max, need to add new one
                    expanding_range = Some(r + 1);
                    good_range.insert(r + 1, *adding_range);
                    continue;
                }

                if adding_range.1 >= active_range.0 {
                    expanding_range = Some(r + 1);
                    good_range.insert(r, *adding_range);
                    continue;
                }
            }

            ri -= 1;
        }

        good_range.insert(0, *adding_range);
    }

    for i in &good_range {
        part2 += i.1 - i.0 + 1;
    }

    println!("Part 2: {}", part2);
}
