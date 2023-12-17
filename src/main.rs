use std::env;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let flags: Vec<String> = env::args().collect();
    if flags.len() < 3 {
        panic!("input error: please input searchstring and filepath")
    }

    let query = &flags[1];
    let path = File::open(&flags[2]).unwrap();

    let _reader = BufReader::new(&path);

    println!("Searching for {}, in {:?}", query, path);
}
