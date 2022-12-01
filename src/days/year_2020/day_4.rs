#![allow(dead_code, unused)]

use std::collections::BTreeMap;

pub fn run_day(input: String) {

    let passport_values = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid","cid"];

    let mut part1 = 0;

    let mut passports = input
        .split("\n\n")
        .map(|x| Passport::new(x.to_string()))
        .collect::<Vec<Passport>>();

    //println!("{:#?}", passports);

    let mut invalid = 0;
    for passport in &passports {
        for value in &passport_values {
            if (*value).eq("cid") {
                continue;
            }
            match passport.batch.get(&value.to_string()) {
                Some(_) => (),
                None => {
                    invalid += 1;
                    break;
                },
            }
        }
    }

    part1 = passports.len() - invalid;
    println!("Part 1: {}", part1);

    let mut invalid = 0;
    for passport in &passports {
        'values: for value in &passport_values {
            if value.contains("cid") {
                continue 'values;
            }
            match passport.batch.get(&value.to_string()) {
                Some(val) => {
                    match *value {
                        "byr" => {
                            if val.len() != 4 {     
                                invalid += 1;
                                break;
                            }
                            let val = val.parse::<i32>().unwrap();
                            if val < 1920 || val > 2002 {
                                invalid += 1;
                                break;
                            }
                        },
                        "iyr" => {
                            if val.len() != 4 {     
                                invalid += 1;
                                break;
                            }
                            let val = val.parse::<i32>().unwrap();
                            if val < 2010 || val > 2020 {
                                invalid += 1;
                                break;
                            }
                        },
                        "eyr" => {
                            if val.len() != 4 {     
                                invalid += 1;
                                break;
                            }
                            let val = val.parse::<i32>().unwrap();
                            if val < 2020 || val > 2030 {
                                invalid += 1;
                                break;
                            }
                        },
                        "hgt" => {
                            if !val.ends_with("cm") && !val.ends_with("in") {
                                invalid += 1;
                                break;
                            }

                            match val[..val.len() - 2].parse::<i32>() {
                                Ok(new_val) => {
                                    match &(*val)[val.len() - 3..] {
                                        "cm" => {
                                            if new_val < 150 || new_val > 193 {
                                                invalid += 1;
                                                break;
                                            }
                                        },
                                        "in" => {
                                            if new_val < 59 || new_val > 76 {
                                                invalid += 1;
                                                break;
                                            }
                                        }
                                        _ => ()
                                    }
                                },
                                Err(_) => {  
                                    invalid += 1;
                                    break;
                                },
                            }
                        },
                        "hcl" => {
                            if !val.starts_with("#") {
                                invalid += 1;
                                break;
                            }

                            if val.len() != 7 {
                                invalid += 1;
                                break;
                            }
                            let mut iter = val.chars();
                            iter.next();

                            for c in iter {
                                match c {
                                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c' | 'd' | 'e' | 'f' => (),
                                    _ => {
                                        invalid += 1;
                                        break 'values;
                                    }
                                }
                            }
                        },
                        "ecl" => {
                            match val.as_str() {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                                _ => {
                                    invalid += 1;
                                    break;
                                }
                            }
                        },
                        "pid" => {
                            if val.len() != 9 {
                                invalid += 1;
                                break;
                            }

                            match val.parse::<i64>() {
                                Ok(_) => (),
                                Err(_) => {
                                    invalid += 1;
                                    break;
                                },
                            }
                        },
                        _ => {
                            invalid += 1;
                            break;
                        },
                    }
                },
                None => {
                    invalid += 1;
                    break;
                },
            }
        }
    }

    let mut part2 = passports.len() - invalid;
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Passport {
    batch: BTreeMap<String, String>,
}

impl Passport {
    fn new(batched_input: String) -> Self {
        let mut map = BTreeMap::new();

        batched_input.split(&[' ', '\n'][..]).for_each(|x| {
            map.insert(
                x.split(':').next().unwrap().to_string(),
                x.split(':').next_back().unwrap().to_string(),
            );
        });

        Self { batch: map }
    }
}
