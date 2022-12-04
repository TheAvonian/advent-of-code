mod days;

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

use clap::{command, Parser, Subcommand, ValueEnum};
use days::run_day;
use serde::{Deserialize, Serialize};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("No arguments were provided, please use -h or --help for more info.");
            return;
        }
        Some(_) => (),
    }

    // user loading done

    let command = match cli.command {
        Some(val) => val,
        None => panic!("This will never happen"),
    };

    match command {
        Commands::New { day, year } => {
            create_day(year as u16, day as u8);
        }
        Commands::Run { day, year, sample } => {
            if !Path::new(&format!("./src/inputs/day{}_{}.txt", day, year)).exists() {
                println!(
                    "PLEASE MAKE FILE './src/inputs/day{}_{}.txt' CONTAIN THE PROPER PUZZLE INPUT.",
                    day, year
                );
                return;
            }
            let str_input = match sample {
                Some(val) if val == true => {
                    fs::read_to_string(format!("./src/inputs/day{}_{}sample.txt", day, year))
                        .unwrap()
                }
                _ => fs::read_to_string(format!("./src/inputs/day{}_{}.txt", day, year)).unwrap(),
            };

            if str_input.is_empty() {
                println!(
                    "There is no puzzle input in file 'src/inputs/day{}_{}.txt'.",
                    day, year
                );
            }

            run_day(year as u16, day as u8, str_input.replace("\r", ""));
        }
    }
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        day: i32,
        year: i32,
    },
    Run {
        day: i32,
        year: i32,
        sample: Option<bool>,
    },
}

#[derive(Clone, ValueEnum)]
enum StatType {
    FastestTime,
    SlowestTime,
    AllTimes,
    LowestStars,
    HighestStars,
    AllStars,
}

#[derive(Serialize, Deserialize)]
struct Data {
    years: HashMap<u16, Vec<u8>>,
}

fn create_day(year: u16, day: u8) {
    let mut data_file = match File::open("./src/days/data_days.json") {
        Err(_) => File::create("./src/days/data_days.json").unwrap(),
        Ok(val) => val,
    };

    if !Path::new(&format!("./src/days/year_{}/", year)).exists() {
        fs::create_dir(format!("./src/days/year_{}/", year)).unwrap();
    }

    let mut year_mod = File::create(format!("./src/days/year_{}/mod.rs", year)).unwrap();

    let mut json_string = String::new();
    data_file.read_to_string(&mut json_string).unwrap();
    let mut json_data: Data = match serde_json::from_str(&json_string) {
        Ok(val) => val,
        Err(_) => Data {
            years: HashMap::new(),
        },
    };

    let mut new_lines: Vec<String> = Vec::new();

    if json_data.years.contains_key(&year) {
        if !json_data.years.get(&year).unwrap().contains(&day) {
            json_data.years.get_mut(&year).unwrap().push(day);
        }
    } else {
        json_data.years.insert(year, vec![day]);
    }

    year_mod
        .write_all(
            &json_data.years[&year]
                .iter()
                .map(|x| format!("pub mod day_{};\n", x))
                .collect::<String>()
                .as_bytes(),
        )
        .unwrap();

    for (yar, _) in json_data.years.iter() {
        new_lines.push(format!("mod year_{};", yar));
    }

    new_lines.push("pub fn run_day(year: u16, day: u8, input: String) {".to_string());

    new_lines.push("    match year {".to_string());

    for (yar, days) in json_data.years.iter() {
        // 2022 => {
        new_lines.push(format!("        {} => {{", yar));
        // match day {
        new_lines.push("            match day {".to_string());

        for day in days.iter() {
            // 1 => {
            new_lines.push(format!("                {} => {{", day));
            // day1_2022::run_day(input);
            new_lines.push(format!(
                "                    year_{}::day_{}::run_day(input);",
                yar, day
            ));
            // }
            new_lines.push("                },".to_string());
        }

        new_lines.push("                _ => ()".to_string());

        // }
        new_lines.push("            }".to_string());
        // }
        new_lines.push("        },".to_string());
    }

    // _ => ()
    new_lines.push("        _ => ()".to_string());

    // }
    new_lines.push("    }".to_string());

    new_lines.push("}".to_string());

    let new_json = serde_json::to_string(&json_data).unwrap();

    // save json

    let mut end_string = "".to_string();
    for x in new_lines {
        end_string += &(x + &"\n".to_string());
    }

    let mut mod_file = File::create("./src/days/mod.rs").unwrap();

    mod_file.write_all(end_string.as_bytes()).unwrap();

    if !Path::new(&format!("./src/days/year_{}/day_{}.rs", year, day)).exists() {
        let mut rs_file = File::create(format!("./src/days/year_{}/day_{}.rs", year, day)).unwrap();

        rs_file.write_all("#![allow(dead_code, unused)]\n\npub fn run_day(input: String) {\n    let mut part1 = 0;\n\n    println!(\"Part 1: {}\", part1);\n    let mut part2 = 0;\n\n    println!(\"Part 2: {}\", part2);\n}".as_bytes()).unwrap();
    }
    File::create(format!("./src/inputs/day{}_{}sample.txt", day, year)).unwrap();
    File::create(format!("./src/inputs/day{}_{}.txt", day, year)).unwrap();

    fs::write("./src/days/data_days.json", new_json).unwrap();
}
