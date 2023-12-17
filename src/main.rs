use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let flags: Vec<String> = env::args().collect();
    if flags.len() != 3 {
        panic!("input error: please provide a query and a filepath");
    }

    let config = Config::new(flags).unwrap();

    let reader = BufReader::new(&config.path);
    reader.lines().enumerate().for_each(|(line_index, line)| {
        if line.unwrap().contains(&config.query) {
            println!("{} found in file at line {}", config.query, line_index);
        }
    });

    println!("Searching for {}, in {:?}", config.query, config.path);
}

struct Config {
    query: String,
    path: File,
}

impl Config {
    fn new(flags: Vec<String>) -> Result<Config, std::io::Error> {
        let query = flags[1].to_string();
        let path = File::open(&flags[2])?;

        Ok(Config { query, path })
    }
}
