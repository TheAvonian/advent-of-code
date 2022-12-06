#![allow(dead_code, unused)]

use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    hash,
    ops::Div,
    time::Instant,
};

use itertools::Itertools;

///
/// NOT DONE
///

pub fn run_day(input: String) {
    let mut part1: i64 = 0;

    let mut variables: HashMap<char, i64> = HashMap::from([('x', 0), ('y', 0), ('z', 0), ('w', 0)]);

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|l| {
            let v_one = Variable::new(&l[4..=4]);
            match &l[..3] {
                "inp" => Instruction::Input(v_one),
                "add" => Instruction::Add(v_one, Variable::new(&l[6..])),
                "mul" => Instruction::Multiply(v_one, Variable::new(&l[6..])),
                "div" => Instruction::Divide(v_one, Variable::new(&l[6..])),
                "mod" => Instruction::Modulo(v_one, Variable::new(&l[6..])),
                "eql" => Instruction::Equals(v_one, Variable::new(&l[6..])),
                _ => panic!("bad"),
            }
        })
        .collect();

    let max = 99999999999999i64;
    let min = 11111111111111i64;

    let mut current = max.to_string();

    let bigboy: u128 = (1..=14).map(|i| 9_u128.pow(i)).sum();
    println!("{}", bigboy);

    let mut i = 0;

    let mut start_time = Instant::now();
    let true_start = Instant::now();

    // less than 4..

    let ran = (1..=9).rev();
    (1..=3).rev().any(|a| {
        ran.clone().any(|b| {
            ran.clone().any(|c| {
                ran.clone().any(|d| {
                    ran.clone().any(|e| {
                        ran.clone().any(|f| {
                            ran.clone().any(|g| {
                                ran.clone().any(|h| {
                                    ran.clone().any(|i| {
                                        ran.clone().any(|j| {
                                            ran.clone().any(|k| {
                                                ran.clone().any(|l| {
                                                    ran.clone().any(|m| {
                                                        ran.clone().any(|n| {
                                                            if is_valid(&instructions, VecDeque::from([a,b,c,d,e,f,g,h,i,j,k,l,m,n])) {
                                                                println!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}{n}");
                                                                return true;
                                                            }

                                                            let o = Instant::now();
                                                            if o.duration_since(start_time)
                                                                .as_secs()
                                                                >= 5
                                                            {
                                                                start_time = o;
                                                                println!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}{n} : {} seconds", o.duration_since(true_start).as_secs());
                                                            }
                                                            false
                                                        })
                                                    })
                                                })
                                            })
                                        })
                                    })
                                })
                            })
                        })
                    })
                })
            })
        })
    });

    /* 'out: for a in ran.clone() {
        for b in ran.clone() {
            for c in ran.clone() {
                for d in ran.clone() {
                    for e in ran.clone() {
                        for f in ran.clone() {
                            for g in ran.clone() {
                                for h in ran.clone() {
                                    for i in ran.clone() {
                                        for j in ran.clone() {
                                            for k in ran.clone() {
                                                for l in ran.clone() {
                                                    for m in ran.clone() {
                                                        for n in ran.clone() {
                                                            if is_valid(&instructions, format!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}{n}")) {
                                                                current = format!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}{n}");
                                                                break 'out;
                                                            }

                                                            let o = Instant::now();
                                                            if o.duration_since(start_time)
                                                                .as_secs()
                                                                >= 5
                                                            {
                                                                start_time = o;
                                                                println!("{a}{b}{c}{d}{e}{f}{g}{h}{i}{j}{k}{l}{m}{n} : {} seconds", o.duration_since(true_start).as_secs());
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } */

    /* 'odne: for x in (0..14).rev() {
        /*
        let radix = 10_i64.pow(x);
        let v = (current / radix) % 10; */
        let mut iterator = (0..9 * (14 - x)).map(|a| a % 9 + 1);


        /* for y in iterator
            .rev()
            .permutations(14 - x)
            .unique()
            .map(|a| a.iter().map(|x| x.to_string()).collect::<String>())
        {
            current.replace_range(x..14, &y);
            if is_valid(&instructions, &variables, current.clone()) {
                break 'odne;
            }
            i += 1;
            if Instant::now().duration_since(start_time).as_secs() > 2 {
                start_time = Instant::now();
                println!("{:0>14} : {} : {}", i, bigboy, x);
            }
        } */
    } */

    /* for x in t {
        current = x.iter().map(|f|f.to_string()).collect::<String>();
    } */

    //println!("{:#?}", last);

    println!("Part 1: {}", current);
    let mut part2 = 0;

    println!("Part 2: {}", part2);
}

enum Variable {
    Literal(i64),
    Variable(char),
}

impl Variable {
    fn new(value: &str) -> Self {
        if let Ok(parsed) = value.parse::<i64>() {
            return Self::Literal(parsed);
        }
        Self::Variable(value.chars().last().unwrap())
    }
}

enum Instruction {
    Input(Variable),
    Add(Variable, Variable),
    Multiply(Variable, Variable),
    Divide(Variable, Variable),
    Modulo(Variable, Variable),
    Equals(Variable, Variable),
}

fn is_valid(instructions: &Vec<Instruction>, value: VecDeque<i32>) -> bool {
    let mut variables: BTreeMap<char, i64> =
        BTreeMap::from([('x', 0), ('y', 0), ('z', 0), ('w', 0)]);
    let mut iterr = value;

    for i in instructions {
        match i {
            Instruction::Input(v) => {
                if let Variable::Variable(v) = v {
                    if let Some(t) = variables.get_mut(v) {
                        *t = iterr.pop_front().unwrap() as i64;
                        // println!("{} set to {}", v, t);
                    }
                }
            }
            Instruction::Add(a, b) => {
                let b: i64 = match b {
                    Variable::Literal(a) => *a,
                    Variable::Variable(a) => variables[a].clone(),
                };

                if let Variable::Variable(a) = a {
                    if let Some(a) = variables.get_mut(a) {
                        *a += b;
                    }
                }
            }
            Instruction::Multiply(a, b) => {
                let b: i64 = match b {
                    Variable::Literal(a) => *a,
                    Variable::Variable(a) => variables[a].clone(),
                };

                if let Variable::Variable(a) = a {
                    if let Some(a) = variables.get_mut(a) {
                        *a *= b;
                    }
                }
            }
            Instruction::Divide(a, b) => {
                let b: i64 = match b {
                    Variable::Literal(a) => *a,
                    Variable::Variable(a) => variables[a].clone(),
                };

                if let Variable::Variable(a) = a {
                    if let Some(a) = variables.get_mut(a) {
                        *a /= b;
                    }
                }
            }
            Instruction::Modulo(a, b) => {
                let b: i64 = match b {
                    Variable::Literal(a) => *a,
                    Variable::Variable(a) => variables[a].clone(),
                };

                if let Variable::Variable(a) = a {
                    if let Some(a) = variables.get_mut(a) {
                        *a %= b;
                    }
                }
            }
            Instruction::Equals(a, b) => {
                let b: i64 = match b {
                    Variable::Literal(a) => *a,
                    Variable::Variable(a) => variables[a].clone(),
                };

                if let Variable::Variable(a) = a {
                    if let Some(a) = variables.get_mut(a) {
                        *a = match b == *a {
                            true => 1,
                            false => 0,
                        };
                    }
                }
            }
        };
    }

    //println!("{:#?}", variables);
    //println!("{:#?}", hashmap);
    variables[&'z'] == 0
}
