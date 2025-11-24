use pico_args::Arguments;
use std::process;
use crate::{y2015};

#[derive(Debug, Clone, Copy)]
pub struct Day {
    pub year: u32,
    pub day: u32,
}

impl Day {
    pub fn new(year: u32, day: u32) -> Self {
        Self { year, day }
    }

    pub fn from_str(s: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() == 2 {
            let year = parts[0].parse::<u32>()?;
            let day = parts[1].parse::<u32>()?;
            Ok(Day::new(year, day))
        } else {
            Err("Day format should be YYYY/DD (e.g., 2024/1)".into())
        }
    }
}

pub enum Command {
    Solve { day: Day, release: bool },
    All { release: bool },
}

impl Command {
    pub fn run(self) {
        match self {
            Command::Solve { day, release } => {
                solve_day(day, release);
            }
            Command::All { release } => {
                println!("Running all solutions");
                if release {
                    println!("(Release mode)");
                }
            }
        }
    }
}

fn solve_day(day: Day, _release: bool) {
    let input_path = format!("input/{}/day{}.txt", day.year, day.day);
    let input = std::fs::read_to_string(&input_path).unwrap_or_else(|_| {
        eprintln!("Warning: input file not found at {}", input_path);
        String::new()
    });
    
    match (day.year, day.day) {
        (2015, 1) => y2015::day1::solve(&input),
        _ => eprintln!("No solver found for {}/{}", day.year, day.day),
    }
}

pub fn parse() -> Result<Command, Box<dyn std::error::Error>> {
    let mut args = Arguments::from_env();

    let command = match args.subcommand()?.as_deref() {
        Some("solve") => {
            let day_str: String = args.free_from_str()?;
            let day = Day::from_str(&day_str)?;
            let release = args.contains("--release");
            Command::Solve { day, release }
        }
        Some("all") => {
            let release = args.contains("--release");
            Command::All { release }
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("\nUsage:");
            eprintln!("  cargo run -- solve <YYYY/DD> [--release]");
            eprintln!("  cargo run -- all [--release]");
            process::exit(1);
        }
        None => {
            eprintln!("No command specified.");
            eprintln!("\nUsage:");
            eprintln!("  cargo run -- solve <YYYY/DD> [--release]");
            eprintln!("  cargo run -- all [--release]");
            process::exit(1);
        }
    };

    Ok(command)
}

