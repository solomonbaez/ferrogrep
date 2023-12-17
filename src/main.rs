use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{Error, ErrorKind};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|e| {
        println!("failed to parse arguments - {}", e);
        std::process::exit(1)
    });

    let contents = BufReader::new(&config.file).lines().enumerate();
    contents.for_each(|(line_index, line)| {
        if line.unwrap().contains(&config.query) {
            println!("{} found in file at line {}", config.query, line_index);
        }
    });

    println!("Searching for {}, in {:?}", config.query, config.file);
}

struct Config {
    query: String,
    file: File,
}

impl Config {
    fn build(args: Vec<String>) -> Result<Config, std::io::Error> {
        if args.len() != 3 {
            return Err(Error::new(
                ErrorKind::Other,
                "input error: please provide a query string and a filepath",
            ));
        }
        let query = args[1].to_string();

        match File::open(&args[2]) {
            Ok(file) => Ok(Config { query, file }),
            Err(e) => Err(e),
        }
    }
}
