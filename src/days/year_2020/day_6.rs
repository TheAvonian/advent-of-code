#![allow(dead_code, unused)]

use std::collections::{HashMap, HashSet};

pub fn run_day(input: String) {
    let groups = input
        .split("\n\n")
        .map(|x| Group::new(x.to_string()))
        .collect::<Vec<Group>>();

    let mut part1 = 0;

    for group in &groups {
        let mut answers: HashSet<char> = HashSet::new();
        for person in &group.people {
            for answer in &person.answered_questions {
                answers.insert(*answer);
            }
        }

        part1 += answers.len();
    }

    println!("Part 1: {}", part1);
    let mut part2 = 0;

    for group in &groups {
        let mut answers: HashMap<char, usize> = HashMap::new();
        for person in &group.people {
            for answer in &person.answered_questions {
                answers.insert(
                    *answer,
                    match answers.get(answer) {
                        Some(val) => val,
                        None => &0,
                    } + 1,
                );
            }
        }
        for (answer, amount) in answers {
            if amount == group.people.len() {
                part2 += 1;
            }
        }
    }

    println!("Part 2: {}", part2);
}

struct Group {
    people: Vec<Person>,
}

struct Person {
    answered_questions: Vec<char>,
}

impl Group {
    fn new(input: String) -> Self {
        Self {
            people: input
                .split('\n')
                .map(|x| Person::new(x.to_string()))
                .collect(),
        }
    }
}

impl Person {
    fn new(input: String) -> Self {
        Self {
            answered_questions: input.chars().collect(),
        }
    }
}
