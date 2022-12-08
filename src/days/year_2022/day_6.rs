use itertools::Itertools;

pub fn run_day(input: String) {
    let f = |v: usize| {
        input
            .as_bytes()
            .windows(v)
            .position(|a| a.iter().all_unique())
            .unwrap()
            + v
    };

    println!("Part 1: {}", f(4));

    println!("Part 2: {}", f(14));
}
