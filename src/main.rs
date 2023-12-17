use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let flags: Vec<String> = env::args().collect();
    if flags.len() < 3 {
        panic!("input error: please input searchstring and filepath")
    }

    let query = &flags[1];
    let path = File::open(&flags[2]).unwrap();

    let reader = BufReader::new(&path);
    reader.lines().enumerate().for_each(|(line_index, line)| {
        if line.unwrap().contains(query) {
            println!("{} found in file at line {}", query, line_index);
        }
    });

    println!("Searching for {}, in {:?}", query, path);
}
