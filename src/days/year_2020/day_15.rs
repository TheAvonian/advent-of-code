#![allow(dead_code, unused)]

use std::{collections::HashMap, time::Instant};

pub fn run_day(input: String) {

    let input_vec: Vec<i32> = input.split(',').map(| x | x.parse::<i32>().unwrap()).collect();

    let size = input_vec.len();

    println!("Input: {}", input);
    let mut part1 = 0;

    let mut game = Game::new(&input_vec);

    let now = Instant::now();
    let mut max = 2020;
    for x in (size + 1..=max) {
        part1 = game.take_turn(x as i32);
    }
    let elapsed_time = now.elapsed();

    println!("Part 1: {}", part1);
    println!("Part 1 took {:.2} seconds", elapsed_time.as_secs_f32());

    let now = Instant::now();

    let mut part2 = 0;

    let mut new_max = 30000000;

    for x in (max + 1..=new_max) {
        part2 = game.take_turn(x as i32);
        /* if x % 10000 == 0 {
            println!("{0:<10}{1:>5.2}%",part2, x as f32 / new_max as f32 * 100.0);
        } */
    }

    let elapsed_time = now.elapsed();

    println!("Part 2: {}", part2);
    println!("Part 2 took {:.2} seconds", elapsed_time.as_secs_f32());
}


struct Game {
    last_val: i32,
    turns: HashMap<i32, i32>
}

impl Game {
    fn new(starting: &Vec<i32>) -> Self {
        let mut turns: HashMap<i32, i32> = HashMap::new();
        for (i, val) in starting.into_iter().enumerate() {
            if i == starting.len() - 1 {
                break;
            }
            turns.insert(*val, i as i32 + 1);
        }
        Self {
            last_val: starting[starting.len() - 1],
            turns
        }
    }

    fn take_turn(&mut self, turn_number: i32) -> i32 {
        let mut end_val = 0;
        if self.turns.contains_key(&self.last_val) {
            let val = self.turns[&self.last_val];
            end_val = turn_number - 1 - val;
        }

        self.turns.insert(self.last_val, turn_number - 1);

        self.last_val = end_val;
        end_val
    }
}