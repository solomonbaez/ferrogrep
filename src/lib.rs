use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub struct Grep {
    query: String,
    path: String,
    contents: Vec<String>,
}

impl Grep {
    pub fn build(args: Vec<String>) -> Result<Grep, std::io::Error> {
        if args.len() < 3 {
            return Err(Error::new(
                ErrorKind::Other,
                "input error: please provide: CASE_SENSITIVE env, query string, and filepath",
            ));
        }
        let query = args[1].to_string();
        let path = args[2].to_string();
        let case_sensitive = std::env::var("CASE_SENSITIVE").is_ok();

        match File::open(&path) {
            Ok(file) => {
                let contents: Vec<String> = BufReader::new(file)
                    .lines()
                    .map(|line| {
                        if case_sensitive {
                            line.unwrap()
                        } else {
                            line.unwrap().to_lowercase()
                        }
                    })
                    .collect();

                Ok(Grep {
                    query,
                    path,
                    contents,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn run(&self) -> Result<Vec<(&str, usize)>, std::io::Error> {
        println!("Searching for {}, in {}", &self.query, &self.path);

        let instances: Vec<(&str, usize)> = self
            .contents
            .iter()
            .enumerate()
            .filter(|(_line_index, line)| line.contains(&self.query))
            .map(|(line_index, line)| (line.as_str(), line_index))
            .collect();

        match instances.len() {
            0 => Err(Error::new(
                ErrorKind::Other,
                format!("0 instances of {} found", &self.query),
            )),
            _ => Ok(instances),
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
        let test_config = Grep {
            query,
            path,
            contents,
        };

        assert_eq!(vec![("hello", 0 as usize)], test_config.run().unwrap());
    }
    #[test]
    fn no_result_fails() {
        let query = "goodbye".to_string();
        let path = "test".to_string();
        let contents: Vec<String> = vec!["hello".to_string(), "hallo".to_string()];
        let test_config = Grep {
            query,
            path,
            contents,
        };

        let expected_error = Error::new(
            ErrorKind::Other,
            format!("0 instances of {} found", &test_config.query),
        );
        assert_eq!(
            expected_error.to_string(),
            test_config.run().unwrap_err().to_string()
        );
    }
}
