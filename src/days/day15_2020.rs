#![allow(dead_code, unused)]

pub fn run_day(input: String) {

    let input_vec: Vec<i32> = input.split(',').map(| x | x.parse::<i32>().unwrap()).collect();

    println!("Input: {}", input);
    let mut part1 = 0;

    let mut game = Game::new(input_vec);

    let mut max = 2020 - 3;
    for x in (1..=max) {
        part1 = game.take_turn();
    }

    println!("Part 1: {}", part1);

    
    let part2 = 0;
    println!("Part 2: {}", part2);
}

struct Game {
    turns: Vec<i32>
}

impl Game {
    fn new(starting: Vec<i32>) -> Self {
        Self {
            turns: starting
        }
    }

    fn take_turn(&mut self) -> i32 {
        let current_val = self.turns[self.turns.len() - 1];
        let mut end_val = 0;
        if self.turns[0..self.turns.len() - 1].contains(&current_val) {

            // 0 3 6 
            // 0 3 6 0
            // 0 3 6 0 != 3 = + 1 = 3
            // 5 1 1 0 5 1 23 0 != 4 = + 1 = 4

            end_val = (self.turns[0..self.turns.len() - 1].iter().rev().position(|&x| x == current_val).unwrap()) as i32 + 1;
        }
        self.turns.push(end_val);
        end_val
    }
}