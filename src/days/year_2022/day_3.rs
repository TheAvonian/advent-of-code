use std::collections::HashMap;

pub fn run_day(input: String) {
    let char_map: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, letter)| (letter, i + 1))
        .collect();

    let sacks: Vec<&str> = input.lines().collect();

    let part1: i32 = sacks
        .iter()
        .map(|s| {
            char_map[&s
                .chars()
                .nth(
                    s[..s.len() / 2]
                        .find(|c| s[s.len() / 2..].contains(c))
                        .unwrap(),
                )
                .unwrap()] as i32
        })
        .sum();

    println!("Part 1: {}", part1);

    let part2: i32 = sacks
        .chunks_exact(3)
        .map(|a| {
            char_map[&a[0]
                .chars()
                .find(|v| a[1].contains(*v) && a[2].contains(*v))
                .unwrap()] as i32
        })
        .sum();

    println!("Part 2: {}", part2);
}
