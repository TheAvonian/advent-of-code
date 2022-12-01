
///
/// NOT DONE
/// 

pub fn run_day(input: String) {
    let mut predicates: Vec<Box<dyn FnMut(i32) -> bool>> = Vec::new();

    let binding = input.replace("\r", "");
    let all_lines: Vec<&str> = binding.split("\n").collect();

    let mut done_ones = Vec::new();

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

        let split_line = move |t: i32| {
            (t <= first_high && t >= first_low) || (t <= second_high && t >= second_low)
        };
        let b = Box::from(split_line);
        predicates.push(b);
    }
    let index = all_lines
        .iter()
        .position(|&y| y.contains(&"tickets:"))
        .unwrap();
    let mut nearby_ticket_vals = Vec::new();

    let ticket_size = all_lines[index + 1].split(',').collect::<Vec<&str>>().len();

    for x in index + 1..all_lines.len() {
        for y in all_lines[x].split(',') {
            match y.parse::<i32>() {
                Ok(val) => nearby_ticket_vals.push(val),
                Err(_) => (),
            };
        } 
    }

    for i in &nearby_ticket_vals {
        let mut l = 0;
        for pred in &mut predicates {
            if !pred.as_mut()(*i) {
                l += 1;
                continue;
            }
        }
        if l == predicates.len() {
            part1 += *i;
        }
    }

    println!("Part 1: {}", part1);

    let ticket_tickets = nearby_ticket_vals.chunks(ticket_size);

    let mut tickettts = Vec::new();

    'outer: for tick_in in ticket_tickets.clone() {
        // crr row swap to col
        for ro in tick_in {
            let mut l = 0;
            for pred in &mut predicates {
                if !pred.as_mut()(*ro) {
                    l += 1;
                }
            }
            if l == predicates.len() {
                continue 'outer;
            }
        }
        tickettts.push(Vec::new());
        tickettts.last_mut().unwrap().append(&mut tick_in.to_vec());
    }

    let mut pred_bools: Vec<Vec<TicketBools>> = Vec::new();

    let mut n = 0;
    for pred in &mut predicates {
        pred_bools.push(Vec::new());
        'test: for col_pos in 0..ticket_size {
            let mut temp = Vec::new();
            for row in tickettts.iter() {
                let v = pred.as_mut()(row[col_pos]);
                if !v {
                    continue 'test;
                }
                temp.push(v);
            }

            pred_bools.last_mut().unwrap().push(TicketBools {
                id: col_pos,
                pred_id: n,
            });
        }
        n += 1;
    }

    let mut part2 = 0;

    let mut removal = Vec::new();

    while !pred_bools.is_empty() {
        for i in (0..pred_bools.len()).rev() {
            if pred_bools[i].len() == 1 {
                done_ones.push(PredicateBools {
                    id: pred_bools[i][0].pred_id,
                    ticket: pred_bools[i][0].clone(),
                });
                removal.push(pred_bools[i][0].id);
                pred_bools.remove(i);
            }
        }

        for i in (0..pred_bools.len()).rev() {
            for x in &removal {
                for y in (0..pred_bools[i].len()).rev() {
                    if pred_bools[i][y].id == *x {
                        pred_bools[i].remove(y);
                    }
                }
            }
        }
    }

    done_ones.sort_by(|x, y| x.id.cmp(&y.id));

    let my_tick_pos = all_lines
        .iter()
        .position(|x| x.contains(&"your ticket"))
        .unwrap();
    let my_ticket: Vec<i32> = all_lines
        .iter()
        .skip(my_tick_pos + 1)
        .take(1)
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for x in 0..6 {
        part2 *= my_ticket[done_ones[x].ticket.id] as i64;
    }

    println!("Part 2: {}", part2);
}

#[derive(Clone)]
struct PredicateBools {
    id: usize,
    ticket: TicketBools,
}

#[derive(Clone, Debug)]
struct TicketBools {
    id: usize,
    pred_id: usize,
}
