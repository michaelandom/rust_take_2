use minigrap::Config;
use std::env;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("search word {}", config.query);
    println!("file {}", config.filename);

    if let Err(e) = minigrap::run(config) {
        println!("problem reading file: {}", e);
        process::exit(1);
    }
}
