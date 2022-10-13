#![allow(dead_code, unused)]

pub fn run_day(input: String) {

    let mut predicates: Vec<Box<dyn FnMut(i32) -> bool>> = Vec::new();

    let binding = input.replace("\r", "");
    let all_lines: Vec<&str> = binding.split("\n").collect();

    let mut part1 = 0;

    for x in &all_lines {
        if *x == "" || x.is_empty() || x.len() == 0 || x.chars().nth(0) == None || !x.chars().nth(0).unwrap().is_alphanumeric() {
            break;
        }
        let new_line: String = x.chars().skip_while(|c| *c != ':').skip(2).collect();
        let first_low: i32 = new_line.chars().take_while(|c| *c != '-').collect::<String>().parse::<i32>().unwrap();
        let first_high: i32 = new_line.chars().skip_while(|c| *c != '-').skip(1).take_while(|c| *c != ' ').collect::<String>().parse::<i32>().unwrap();
        let second_bit: String = new_line.chars().skip_while(|c| *c != 'r').skip(2).collect();
        let second_low: i32 = second_bit.chars().take_while(|c| *c != '-').collect::<String>().parse::<i32>().unwrap();
        let second_high: i32 = second_bit.chars().skip_while(|c| *c != '-').skip(1).take_while(|c| c.is_numeric()).collect::<String>().parse::<i32>().unwrap();

        //println!("{first_low:>5}-{first_high:<5} || {second_low:>5}-{second_high:<5}");
 
        let split_line = (move |t: i32| (t <= first_high && t >= first_low) || (t <= second_high && t >= second_low));
        predicates.push(Box::from(split_line));
    }
    let index = all_lines.iter().position(|&y| y.contains(&"tickets:")).unwrap();
    let mut nearby_ticket_vals = Vec::new();
    for x in (index + 1..all_lines.len()) {
        for y in all_lines[x].split(',') {
            let new_y = y.chars().take_while(|x|x.is_alphanumeric()).collect::<String>();
            match y.parse::<i32>() {
                Ok(val) => nearby_ticket_vals.push(val),
                Err(err) => (),
            };
        }
    }

    for x in &nearby_ticket_vals {
        let mut l = 0;
        let mut invalid = false;
        for pred in &mut predicates {
            if !pred.as_mut()(*x) {
                l+=1;
                continue;
            }
            break;
        }
        if l == predicates.len() {
            part1 += *x;
        }
    }



    println!("Part 1: {}", part1);



    let mut part2 = 0;
    println!("Part 2: {}", part2);
}