mod args;
mod y2015;
mod y2025;

fn main() {
    match args::parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(command) => command.run(),
    };
}


