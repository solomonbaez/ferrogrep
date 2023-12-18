use ferrogrep::Config;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(args).unwrap_or_else(|e| {
        println!("failed to parse arguments - {}", e);
        std::process::exit(1)
    });

    let instances = config.run().unwrap_or_else(|e| {
        println!("failed - {}", e);
        std::process::exit(1)
    });

    println!("success - (line, location) - {:?}", instances);
}
