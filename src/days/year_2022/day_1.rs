use itertools::Itertools;

pub fn run_day(input: String) {
    let elves = input
        .split("\n\n")
        .map(|e| e.lines().map(|x| x.parse::<u32>().unwrap()).sum::<u32>());

    let part1: u32 = elves.clone().max().unwrap();

    println!("Part 1: {part1}");

    let part2: u32 = elves.sorted().rev().take(3).sum();

    println!("Part 2: {part2}");
}
