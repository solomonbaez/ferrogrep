use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct Config {
    query: String,
    path: String,
    file: File,
}

impl Config {
    pub fn build(args: Vec<String>) -> Result<Config, std::io::Error> {
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

    pub fn run(&self) -> Result<(), std::io::Error> {
        println!("Searching for {}, in {}", &self.query, &self.path);

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
