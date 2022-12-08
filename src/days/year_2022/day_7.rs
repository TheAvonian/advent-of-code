use std::collections::BTreeMap;

pub fn run_day(input: String) {
    let (map, _) = input.lines().fold(
        (BTreeMap::<String, u64>::new(), Vec::<String>::new()),
        |(mut dirs, mut path), line| {
            if line.starts_with("$ cd") {
                match line.get(5..).unwrap() {
                    ".." => {
                        path.pop();
                    }
                    "/" => {
                        dirs.entry("/".to_string()).or_insert(0);
                        path = vec!["/".to_string()];
                    }
                    v => {
                        path.push(v.to_string());
                        dirs.entry(
                            path.iter()
                                .flat_map(|a| a.chars())
                                .chain(v.chars())
                                .collect(),
                        )
                        .or_insert(0);
                    }
                }
            } else if line.starts_with("dir") {
                dirs.entry(
                    path.iter()
                        .flat_map(|a| a.chars())
                        .chain(line.get(4..).unwrap().chars())
                        .collect(),
                )
                .or_insert(0);
            } else if !line.starts_with("$") {
                let size = line
                    .chars()
                    .take_while(|c| !c.eq(&' '))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                for x in (1..=path.len()).rev() {
                    dirs.entry(path.iter().take(x).flat_map(|a| a.chars()).collect())
                        .and_modify(|f| *f += size);
                }
            }

            (dirs, path)
        },
    );
    let sizes = map.iter().map(|(_, v)| *v).collect::<Vec<u64>>();

    let part1: u64 = sizes.iter().filter(|&&a| a <= 100_000).sum();

    println!("Part 1: {}", part1);

    let space_needed: u64 = 30_000_000 - (70_000_000 - map["/"]);

    let part2: u64 = *sizes.iter().filter(|&&a| a >= space_needed).min().unwrap();

    println!("Part 2: {}", part2);
}
