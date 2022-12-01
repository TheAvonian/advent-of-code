pub fn run_day(input: String) {
    let mut elves = input.split('\n').fold(Vec::new(), |mut list, line| {
        if line.is_empty() {
            list.push(0);
            return list;
        }

        if let Some(last) = list.last_mut() {
            *last += line.parse::<u32>().unwrap();
        }
        list
    });

    let part1: u32 = *elves.iter().max().unwrap();

    println!("Part 1: {part1}");

    elves.sort();

    let part2: u32 = elves.iter().rev().take(3).sum();

    println!("Part 2: {part2}");
}
