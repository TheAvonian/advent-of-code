use clap::{arg, command, Parser, Subcommand, ValueEnum};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        None => {
            println!("No arguments were provided, please use -h or --help for more info.");
            return;
        }
        Some(_) => (),
    }

    
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New { day: i32, year: i32 },
    Run { 
        day: i32, 
        year: i32, 
        #[arg(value_enum)]
        part: Part 
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Part {
    One,
    Two
}
