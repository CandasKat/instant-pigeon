use std::env;
use std::fmt::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process; // Import the process module

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // This is now valid because of the import
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = instant_pigeon::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }


}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}


