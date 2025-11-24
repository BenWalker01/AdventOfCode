mod args;
mod y2015;
mod y2023;
mod y2024;

fn main() {
    match args::parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(command) => command.run(),
    };
}


