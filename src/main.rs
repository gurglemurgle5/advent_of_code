use std::process::exit;

use advent_of_code::get_day;
use aoc_utils::InputManager;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run { year: u16, day: u8 },
    SetToken { token: String },
    GetInput { year: u16, day: u8 },
}

fn main() {
    let cli = Cli::parse();
    let mut input_manager = InputManager::new();
    match cli.command {
        Commands::Run { year, day } => {
            if let Some(day_handle) = get_day(year, day) {
                let input = input_manager.get_input(year, day);
                let day = day_handle.init_day(input);
                println!("Part 1: {}", day.part1());
                println!("Part 2: {}", day.part2());
            } else {
                eprintln!("Day specificed not implimented!");
                exit(1);
            }
        }
        Commands::SetToken { token } => input_manager.set_token(token),
        Commands::GetInput { year, day } => {
            if input_manager.token().is_none() {
                eprintln!("ERROR: No token set!");
                exit(1);
            }
            let input = input_manager.get_input(year, day);
            print!("{input}");
        }
    }
}
