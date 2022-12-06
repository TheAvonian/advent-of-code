#![allow(dead_code, unused)]

use itertools::Itertools;

pub fn run_day(input: String) {
    let mut part1 = 0;

    let new_input = input.split("\n\n").collect::<Vec<_>>();

    let mut tru = Vec::new();
    for x in new_input[0].lines().last().unwrap().chars() {
        if x.is_whitespace() || x.is_ascii_whitespace() {
            continue;
        }

        tru.push(x);
    }

    let mut stacks = Vec::new();
    let p = new_input[0].lines().last().unwrap();
    for x in 0..tru.len() {
        let mut itere = new_input[0].lines();
        stacks.push(Vec::new());
        for y in 0..new_input[0].lines().count() - 1 {
            let line = itere.next().unwrap();

            let charrr = line.chars().nth(4 * x + 1).unwrap();
            if charrr.is_whitespace() || charrr.is_ascii_whitespace() {
                continue;
            }
            stacks[x].push(charrr);
        }
    }
    // 5 12 17
    for x in new_input[1].lines() {
        let amount: i32 = x
            .chars()
            .skip(5)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();
        let from: i32 = x
            .chars()
            .skip(8).skip_while(|c| !c.eq(&'m'))
            .skip(2)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();
        let to: i32 = x
            .chars()
            .skip(15).skip_while(|c| !c.eq(&'o'))
            .skip(2)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();

        let mut temp_stack = Vec::new();
        for x in 0..amount {
            temp_stack.push(stacks[from as usize - 1 as usize][0]);
            stacks[from as usize - 1 as usize].remove(0);
        }
        for x in 0..amount {
            stacks[to as usize - 1 as usize].insert(0, temp_stack[x as usize]);
        }
    }

    let mut asss = 0;
    let mut largest = 0;

    let mut v = Vec::new();
    for x in 0..stacks.len() {
        v.push(stacks[x][0]);
    }


    println!("{:#?}", v);

    println!("{:#?}", largest);

    println!("Part 1: {}", part1);
    let mut part2 = 0;

    let mut stacks = Vec::new();
    let p = new_input[0].lines().last().unwrap();
    for x in 0..tru.len() {
        let mut itere = new_input[0].lines();
        stacks.push(Vec::new());
        for y in 0..new_input[0].lines().count() - 1 {
            let line = itere.next().unwrap();

            let charrr = line.chars().nth(4 * x + 1).unwrap();
            if charrr.is_whitespace() || charrr.is_ascii_whitespace() {
                continue;
            }
            stacks[x].push(charrr);
        }
    }
    // 5 12 17
    for x in new_input[1].lines() {
        let amount: i32 = x
            .chars()
            .skip(5)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();
        let from: i32 = x
            .chars()
            .skip(8).skip_while(|c| !c.eq(&'m'))
            .skip(2)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();
        let to: i32 = x
            .chars()
            .skip(15).skip_while(|c| !c.eq(&'o'))
            .skip(2)
            .take_while(|x| x.is_numeric() || x.is_ascii_alphanumeric())
            .collect::<String>()
            .to_string()
            .parse()
            .unwrap();

        let mut temp_stack = Vec::new();
        for x in 0..amount {
            temp_stack.push(stacks[from as usize - 1 as usize][0]);
            stacks[from as usize - 1 as usize].remove(0);
        }
        for x in (0..amount).rev() {
            stacks[to as usize - 1 as usize].insert(0, temp_stack[x as usize]);
        }
    }
    let mut v = Vec::new();
    for x in 0..stacks.len() {
        v.push(stacks[x][0]);
    }


    println!("{:#?}", v);

    println!("Part 2: {}", part2);
}
