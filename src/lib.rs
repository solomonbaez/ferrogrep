use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct Config {
    query: String,
    path: String,
    contents: Vec<String>,
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
            Ok(file) => {
                let contents: Vec<String> = BufReader::new(file)
                    .lines()
                    .map(|line| line.unwrap())
                    .collect();
                Ok(Config {
                    query,
                    path,
                    contents,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&self) -> Result<bool, std::io::Error> {
        println!("Searching for {}, in {}", &self.query, &self.path);

        let mut instances = 0;
        self.contents
            .iter()
            .enumerate()
            .for_each(|(line_index, line)| {
                if line.contains(&self.query) {
                    instances += 1;
                    println!("{} found in file at line {}", self.query, line_index);
                }
            });

        if instances == 0 {
            println!("Failed: no instances of {} found", &self.query);
            Ok(false)
        } else {
            println!(
                "Success: found {} instances of '{}' in file '{}'",
                instances, &self.query, &self.path,
            );
            Ok(true)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn one_result_passes() {
        let query = "hello".to_string();
        let path = "test".to_string();
        let contents: Vec<String> = vec!["hello".to_string(), "hallo".to_string()];
        let test_config = Config {
            query,
            path,
            contents,
        };

        assert_eq!(true, test_config.run().unwrap());
    }
}
