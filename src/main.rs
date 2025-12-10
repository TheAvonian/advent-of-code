mod days;

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::{Parser, Subcommand, ValueEnum};
use days::run_day;

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

fn create_day(year: u16, day: u8) {
    if !Path::new(&format!("./src/days/year_{}/", year)).exists() {
        fs::create_dir(format!("./src/days/year_{}/", year)).unwrap();
    }

    if !Path::new(&format!("./src/days/year_{}/mod.rs", year)).exists() {
        std::fs::write(
            format!("./src/days/year_{}/mod.rs", year),
            format!(
                "use advent_macros::gen_pub_days;

gen_pub_days!({});",
                year
            ),
        )
        .expect("FAILED TO CREATE YEAR MOD");
    }

    if !Path::new("./src/days/mod.rs").exists() {
        std::fs::write(
            "./src/days/mod.rs",
            format!(
                "use advent_macros::gen_switch_years;

gen_switch_years!();"
            ),
        )
        .expect("FAILED FAILEd");
    }

    if !Path::new(&format!("./src/days/year_{}/day_{}.rs", year, day)).exists() {
        let mut rs_file = File::create(format!("./src/days/year_{}/day_{}.rs", year, day)).unwrap();

        rs_file.write_all("#![allow(dead_code, unused)]\n\npub fn run_day(input: String) {\n    let mut part1 = 0;\n\n    println!(\"Part 1: {}\", part1);\n    let mut part2 = 0;\n\n    println!(\"Part 2: {}\", part2);\n}".as_bytes()).unwrap();
    }
    File::create(format!("./src/inputs/day{}_{}sample.txt", day, year)).unwrap();
    File::create(format!("./src/inputs/day{}_{}.txt", day, year)).unwrap();
}
