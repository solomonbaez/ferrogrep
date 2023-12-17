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

    config.run().unwrap_or_else(|e| {
        println!("failed to parse - {}", e);
        std::process::exit(1)
    });
}

struct Config {
    query: String,
    path: String,
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
        let path = args[2].to_string();

        match File::open(&path) {
            Ok(file) => Ok(Config { query, path, file }),
            Err(e) => Err(e),
        }
    }

    fn run(&self) -> Result<(), std::io::Error> {
        println!("Searching for {}, in {:?}", &self.query, &self.file);

        let mut instances = 0;
        let contents = BufReader::new(&self.file).lines().enumerate();
        contents.for_each(|(line_index, line)| {
            if line.unwrap().contains(&self.query) {
                instances += 1;
                println!("{} found in file at line {}", self.query, line_index);
            }
        });

        if instances == 0 {
            return Err(Error::new(
                ErrorKind::Other,
                format!("search error: no instances of {} found", &self.query),
            ));
        } else {
            println!(
                "Success: found {} instances of '{}' in file '{}'",
                instances, &self.query, &self.path,
            );
            Ok(())
        }
    }
}
