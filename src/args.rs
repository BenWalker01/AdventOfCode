use pico_args::Arguments;
use std::process;
use std::path::Path;
use std::fs;
use crate::y2015;

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
    Create { day: Day },
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
            Command::Create { day } => {
                create_day(day);
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
        (2015, 2) => y2015::day2::solve(&input),
        (2015, 3) => y2015::day3::solve(&input),
        (2015, 4) => y2015::day4::solve(&input),
        _ => eprintln!("No solver found for {}/{}", day.year, day.day),
    }
}

fn create_day(day: Day) {
    let src_dir = format!("src/y{}", day.year);
    let src_file = format!("{}/day{}.rs", src_dir, day.day);
    let input_dir = format!("input/{}", day.year);
    let input_file = format!("{}/day{}.txt", input_dir, day.day);
    let mod_file = format!("{}/mod.rs", src_dir);

    // Check if the day file already exists
    if Path::new(&src_file).exists() {
        eprintln!("File already exists: {}", src_file);
        process::exit(1);
    }

    // Create src/yYYYY directory if it doesn't exist
    if !Path::new(&src_dir).exists() {
        fs::create_dir_all(&src_dir).expect("Failed to create source directory");
        
        // Create .keep file
        fs::write(format!("{}/.keep", src_dir), "")
            .expect("Failed to create .keep file");

        // Create mod.rs file
        fs::write(&mod_file, "")
            .expect("Failed to create mod.rs file");
        
        println!("Created directory and mod.rs: {}", src_dir);
        
        // Update main.rs to include the year module
        update_main_rs_for_year(day.year);
    }

    // Create input/YYYY directory if it doesn't exist
    if !Path::new(&input_dir).exists() {
        fs::create_dir_all(&input_dir).expect("Failed to create input directory");
        
        // Create .keep file
        fs::write(format!("{}/.keep", input_dir), "")
            .expect("Failed to create .keep file");
        
        println!("Created directory: {}", input_dir);
    }

    // Copy template to src/yYYYY/dayX.rs
    let template = fs::read_to_string("template")
        .expect("Failed to read template file");
    fs::write(&src_file, template)
        .expect("Failed to create day source file");
    println!("Created: {}", src_file);

    // Create empty input file
    fs::write(&input_file, "")
        .expect("Failed to create input file");
    println!("Created: {}", input_file);

    // Update mod.rs to include the new day module
    let mut mod_content = fs::read_to_string(&mod_file)
        .expect("Failed to read mod.rs file");
    
    let mod_line = format!("pub mod day{};\n", day.day);
    if !mod_content.contains(&mod_line) {
        mod_content.push_str(&mod_line);
        fs::write(&mod_file, mod_content)
            .expect("Failed to update mod.rs file");
        println!("Updated: {}", mod_file);
    }

    // Update args.rs to include the new day solver
    update_args_for_day(day);
}

fn update_main_rs_for_year(year: u32) {
    let main_file = "src/main.rs";
    let mut content = fs::read_to_string(main_file)
        .expect("Failed to read main.rs file");

    let module_line = format!("mod y{};", year);
    if !content.contains(&module_line) {
        // Add the module declaration after the last existing year module
        let insert_after = "mod y2015;";
        let new_modules = format!("mod y2015;\nmod y{};", year);
        content = content.replace(insert_after, &new_modules);
        
        fs::write(main_file, content)
            .expect("Failed to update main.rs file");
        println!("Updated: {}", main_file);
    }
}

fn update_args_for_day(day: Day) {
    let args_file = "src/args.rs";
    let mut content = fs::read_to_string(args_file)
        .expect("Failed to read args.rs file");

    let module_name = format!("y{}", day.year);
    let day_name = format!("day{}", day.day);
    
    // Check if the year module is imported
    let import_line = format!("use crate::{};", module_name);
    if !content.contains(&import_line) {
        // Add the import after the last use crate::yXXXX; line
        // Find the last use crate::yXXXX line
        let lines: Vec<&str> = content.lines().collect();
        let mut insert_at_line = 0;
        
        for (i, line) in lines.iter().enumerate() {
            if line.contains("use crate::y") && !line.contains("//") {
                insert_at_line = i + 1;
            }
        }
        
        if insert_at_line > 0 && insert_at_line <= lines.len() {
            let mut new_lines = lines[..insert_at_line].to_vec();
            new_lines.push(&import_line);
            new_lines.extend_from_slice(&lines[insert_at_line..]);
            content = new_lines.join("\n") + "\n";
        }
    }

    // Find the wildcard pattern in the solve_day function match statement and add the new case
    let old_wildcard = "        _ => eprintln!(\"No solver found for {}/{}\", day.year, day.day),";
    if content.contains(old_wildcard) {
        let new_case = format!("        ({}, {}) => {}::{}::solve(&input),\n{}", 
            day.year, day.day, module_name, day_name, old_wildcard);
        content = content.replace(old_wildcard, &new_case);
        
        fs::write(args_file, content)
            .expect("Failed to update args.rs file");
        println!("Updated: {}", args_file);
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
        Some("create") => {
            let day_str: String = args.free_from_str()?;
            let day = Day::from_str(&day_str)?;
            Command::Create { day }
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("\nUsage:");
            eprintln!("  cargo run -- solve <YYYY/DD> [--release]");
            eprintln!("  cargo run -- all [--release]");
            eprintln!("  cargo run -- create <YYYY/DD>");
            process::exit(1);
        }
        None => {
            eprintln!("No command specified.");
            eprintln!("\nUsage:");
            eprintln!("  cargo run -- solve <YYYY/DD> [--release]");
            eprintln!("  cargo run -- all [--release]");
            eprintln!("  cargo run -- create <YYYY/DD>");
            process::exit(1);
        }
    };

    Ok(command)
}

