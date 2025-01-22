use std::env;
use std::error::Error;
use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1); 
    });
    println!("search word {}", config.query);
    println!("file {}", config.filename);

   if let Err(e) = run(config) {
    println!("problem reading file: {}", e);
    process::exit(1); 
   }
}

fn run(config:Config) -> Result<(),Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    println!("with text:\n{}", content);
    Ok(())
}

struct Config<'a> {
    query: &'a String,
    filename: &'a String,
}

impl<'a> Config<'a> {
    fn new(args: &'a[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("no args found");
        }
        let query = &args[1];
        let filename = &args[2];

        Ok(Config { query, filename })
    }
}
