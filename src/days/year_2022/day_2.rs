
pub fn run_day(input: String) {
    let lines = input
        .lines()
        .map(|x| {
            (
                x.chars().next().unwrap() as i32 - 65,
                x.chars().last().unwrap() as i32 - 88,
            )
        })
        .collect::<Vec<(i32, i32)>>();

    let part1: i32 = lines.iter().map(|(t, y)| checker(*y, *t)).sum();

    println!("Part 1: {}", part1);

    let part2: i32 = lines
        .iter()
        .map(|(t, y)| checker((*y + 2 + *t) % 3, *t))
        .sum();

    println!("Part 2: {}", part2);
}

fn checker(yours: i32, theirs: i32) -> i32 {
    yours
        + match (yours - theirs).rem_euclid(3) {
            0 => 4,
            1 => 7,
            _ => 1,
        }
}
