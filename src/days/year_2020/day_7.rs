#![allow(dead_code, unused)]

use iter_tools::Itertools;
use std::collections::HashMap;

pub fn run_day(input: String) {
    let mut part1 = 0;

    let lines = input.split('\n').collect::<Vec<&str>>();

    let mut bags: HashMap<String, Bag> = HashMap::new();

    for x in &lines {
        let bag_name = x
            .split(' ')
            .take_while(|w| !w.contains(&"bags"))
            .collect::<String>();

        let containers = x.split(' ').skip_while(|w| !w.contains(&"contain")).skip(1);

        // "1|drab|brown||,||3|dotted|crimson||."

        let contains = containers
            .collect::<Vec<&str>>()
            .split(|f| f.contains("bag") || f.contains("bags"))
            .filter(|f| f.len() == 3)
            .map(|f| f.to_vec())
            .collect::<Vec<Vec<&str>>>();

        let mut bag_map = HashMap::new();

        for x in &contains {
            bag_map.insert(x[1].to_owned() + x[2], x[0].parse::<i32>().unwrap());
        }

        bags.insert(
            bag_name.clone(),
            Bag {
                holds: bag_map,
                name: bag_name,
            },
        );
    }

    let holds_shiny = get_what_holds(&bags, "shinygold".to_string(), vec![]);

    println!("Part 1: {}", holds_shiny.len());

    let part2 = get_children_bags(&bags, "shinygold".to_string()) - 1;

    println!("Part 2: {}", part2);
}

fn get_what_holds(
    bags: &HashMap<String, Bag>,
    current_bag: String,
    all_bags: Vec<String>,
) -> Vec<String> {
    if bags
        .iter()
        .filter(|(x, y)| y.holds.contains_key(&current_bag))
        .count()
        == 0
    {
        let mut new_bags = all_bags.clone();
        new_bags.push(current_bag);
        return new_bags;
    }

    bags.iter()
        .filter(|(x, y)| y.holds.contains_key(&current_bag))
        .map(|b| {
            let mut new_bags = all_bags.clone();
            new_bags.push(b.0.clone());
            get_what_holds(&bags, b.0.clone(), new_bags)
        })
        .flatten()
        .unique()
        .collect::<Vec<String>>()
}

fn get_children_bags(bags: &HashMap<String, Bag>, current_bag: String) -> i32 {
    let mut end = 1;
    for x in &bags[&current_bag].holds {
        end += x.1 * get_children_bags(bags, x.0.to_string());
    }
    end
}

#[derive(Debug)]
struct Bag {
    name: String,
    holds: HashMap<String, i32>,
}
