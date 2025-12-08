use std::{hint::black_box, process::exit, time::Instant};

use advent_of_code::get_day;
use aoc_utils::input_manager::InputManager;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command()]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        year: u16,
        day: u8,
    },
    SetToken {
        token: String,
    },
    GetInput {
        year: u16,
        day: u8,
    },
    Bench {
        year: u16,
        day: u8,
        #[arg(short, long, default_value = "10")]
        runs: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut input_manager = InputManager::new();
    match cli.command {
        Commands::Run { year, day } => {
            if let Some(day_handle) = get_day(year, day) {
                let input = input_manager.get_input(year, day);
                let day = day_handle.init_day(&input);
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
        Commands::Bench { year, day, runs } => {
            if let Some(day_handle) = get_day(year, day) {
                let input = input_manager.get_input(year, day);
                let now = Instant::now();
                for _ in 0..runs {
                    let day = black_box(day_handle.init_day(&input));
                    black_box(day.part1());
                    black_box(day.part2());
                }
                let duration = now.elapsed();
                println!("Time elapsed: {:?}", duration / runs);
            } else {
                eprintln!("Day specificed not implimented!");
                exit(1);
            }
        }
    }
}
