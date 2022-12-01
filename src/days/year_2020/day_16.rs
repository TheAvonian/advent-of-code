#![allow(dead_code, unused)]

pub fn run_day(input: String) {
    let mut predicates: Vec<Box<dyn FnMut(i32) -> bool>> = Vec::new();

    let binding = input.replace("\r", "");
    let all_lines: Vec<&str> = binding.split("\n").collect();

    let mut part1 = 0;

    for x in &all_lines {
        if *x == ""
            || x.is_empty()
            || x.len() == 0
            || x.chars().nth(0) == None
            || !x.chars().nth(0).unwrap().is_alphanumeric()
        {
            break;
        }
        let new_line: String = x.chars().skip_while(|c| *c != ':').skip(2).collect();
        let first_low: i32 = new_line
            .chars()
            .take_while(|c| *c != '-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let first_high: i32 = new_line
            .chars()
            .skip_while(|c| *c != '-')
            .skip(1)
            .take_while(|c| *c != ' ')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let second_bit: String = new_line.chars().skip_while(|c| *c != 'r').skip(2).collect();
        let second_low: i32 = second_bit
            .chars()
            .take_while(|c| *c != '-')
            .collect::<String>()
            .parse::<i32>()
            .unwrap();
        let second_high: i32 = second_bit
            .chars()
            .skip_while(|c| *c != '-')
            .skip(1)
            .take_while(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        //println!("{first_low:>5}-{first_high:<5} || {second_low:>5}-{second_high:<5}");

        let split_line = (move |t: i32| {
            (t <= first_high && t >= first_low) || (t <= second_high && t >= second_low)
        });
        predicates.push(Box::from(split_line));
    }
    let index = all_lines
        .iter()
        .position(|&y| y.contains(&"tickets:"))
        .unwrap();
    let mut nearby_ticket_vals = Vec::new();
    for x in (index + 1..all_lines.len()) {
        let inner_ticks: Vec<i32> = all_lines[x]
            .split(',')
            .map(|t| t.parse::<i32>().unwrap())
            .collect();

        nearby_ticket_vals.push(inner_ticks);

        /* for y in all_lines[x].split(',') {
            let new_y = y.chars().take_while(|x|x.is_alphanumeric()).collect::<String>();
            match y.parse::<i32>() {
                Ok(val) => nearby_ticket_vals.push(val),
                Err(err) => (),
            };
        } */
    }

    let mut nearby_valid_tickets = Vec::new();

    for x in &nearby_ticket_vals {
        let mut invalid = false;
        for ticket in x {
            let mut l = 0;
            for pred in &mut predicates {
                if !pred.as_mut()(*ticket) {
                    l += 1;
                    continue;
                }
                break;
            }
            if l == predicates.len() {
                part1 += *ticket;
                invalid = true;
            }
        }

        if !invalid {
            nearby_valid_tickets.push(x);
        }
    }

    //println!("{:?}", nearby_valid_tickets);

    println!("Part 1: {}", part1);

    let mut part2 = 0;

    let mut predicate_order = Vec::new();

    for col_pos in (0..predicates.len()) {
        let mut available_preds: Vec<usize> = (0..predicates.len()).collect();
        for ticket in &nearby_ticket_vals {
            for pred_index in &available_preds.clone() {
                if !predicates[*pred_index].as_mut()(ticket[col_pos]) {
                    available_preds = available_preds
                        .iter()
                        .filter(|f| *f != pred_index)
                        .map(|f| *f)
                        .collect();
                }
            }
        }
        if available_preds.len() > 1 {
            panic!("why do i have so many options");
        }
        predicate_order.push(available_preds[0]);
    }

    println!("{:?}", predicate_order);

    println!("Part 2: {}", part2);
}
