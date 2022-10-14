#![allow(dead_code, unused)]

pub fn run_day(input: String) {
    let mut part1 = 0;

    let mut lows: Vec<usize> = Vec::new();
    let mut highs: Vec<usize> = Vec::new();
    let mut letters: Vec<char> = Vec::new();
    let mut passwords: Vec<&str> = Vec::new();
    let binding = input.replace("\r", "");
    let all_lines: Vec<&str> = binding.split('\n').collect();

    for x in &all_lines {
        let low = x.split('-').collect::<Vec<&str>>()[0]
            .parse::<usize>()
            .unwrap();
        let high = x.split(&['-', ' '][..]).collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        lows.push(low);
        highs.push(high);

        let letter = x.split(&[' ', ':'][..]).collect::<Vec<&str>>()[1]
            .chars()
            .next()
            .unwrap();

        letters.push(letter);

        let password = x.split(' ').collect::<Vec<&str>>()[2];

        passwords.push(password);
    }

    for x in (0..passwords.len()) {
        if part1_predicate(letters[x], passwords[x], lows[x], highs[x]) {
            part1 += 1;
        }
    }

    println!("Part 1: {}", part1);

    let mut lows: Vec<usize> = Vec::new();
    let mut highs: Vec<usize> = Vec::new();
    let mut letters: Vec<char> = Vec::new();
    let mut passwords: Vec<&str> = Vec::new();
    let binding = input.replace("\r", "");
    let all_lines: Vec<&str> = binding.split('\n').collect();

    for x in &all_lines {
        let low = x.split('-').collect::<Vec<&str>>()[0]
            .parse::<usize>()
            .unwrap();
        let high = x.split(&['-', ' '][..]).collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();

        lows.push(low);
        highs.push(high);

        let letter = x.split(&[' ', ':'][..]).collect::<Vec<&str>>()[1]
            .chars()
            .next()
            .unwrap();

        letters.push(letter);

        let password = x.split(' ').collect::<Vec<&str>>()[2];

        passwords.push(password);
    }

    let mut part2 = 0;

    for x in (0..passwords.len()) {
        if part2_predicate(letters[x], passwords[x], lows[x], highs[x]) {
            part2 += 1;
        }
    }

    println!("Part 2: {}", part2);
}

fn part1_predicate(char_to_find: char, word: &str, low: usize, high: usize) -> bool {
    let count = word.chars().filter(|c| *c == char_to_find).count();
    count <= high && count >= low
}

fn part2_predicate(char_to_find: char, word: &str, low: usize, high: usize) -> bool {
    word.get((low - 1)..(low)).unwrap().contains(char_to_find)
        ^ word.get((high - 1)..(high)).unwrap().contains(char_to_find)
}
