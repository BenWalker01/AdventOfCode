mod args;
mod y2015;

fn main() {
    match args::parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(command) => command.run(),
    };
}


