pub fn run_day(input: String) {
    let elf_pairs: Vec<_> = input
        .lines()
        .map(|x| {
            let g = |string: &str| -> i32 { string.parse::<i32>().unwrap() };

            let split = x.split(",").collect::<Vec<&str>>();
            let new_split = split
                .iter()
                .map(|l| l.split("-").collect::<Vec<&str>>())
                .collect::<Vec<Vec<&str>>>();
            (
                g(new_split[0][0])..=g(new_split[0][1]),
                g(new_split[1][0])..=g(new_split[1][1]),
            )
        })
        .collect();

    let part1 = elf_pairs
        .iter()
        .filter(|(a, b)| a.clone().all(|v| b.contains(&v)) || b.clone().all(|v| a.contains(&v)))
        .count();

    println!("Part 1: {}", part1);

    let part2 = elf_pairs
        .iter()
        .filter(|(a, b)| a.clone().any(|v| b.contains(&v)))
        .count();

    println!("Part 2: {}", part2);
}
