#![allow(dead_code, unused)]

pub fn run_day(input: String) {
    let mut part1: i64 = 0;

    let (mut x, mut y, mut z, mut w) = (
        Variable { value: 0 },
        Variable { value: 0 },
        Variable { value: 0 },
        Variable { value: 0 },
    );



    println!("Part 1: {}", part1);
    let mut part2 = 0;

    println!("Part 2: {}", part2);
}

struct Variable {
    value: i64,
}
